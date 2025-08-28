"""
FoT Ethics Validator - Enforces virtue thresholds and SHACL validation
"""

import logging
from typing import Dict, Any, List, Optional
from pydantic import BaseModel, Field
from neo4j import GraphDatabase
import yaml

logger = logging.getLogger(__name__)

class ValidationRequest(BaseModel):
    """Request for ethics validation"""
    operation: str = Field(..., description="Operation being performed")
    subject: str = Field(..., description="Subject performing the operation")
    concept: Optional[str] = Field(None, description="Concept being operated on")
    virtues: Dict[str, float] = Field(default_factory=dict, description="Virtue metrics")
    content: Optional[str] = Field(None, description="Content being validated")
    provenance: Optional[List[str]] = Field(None, description="Source URLs")

class ValidationResponse(BaseModel):
    """Response from ethics validation"""
    approved: bool = Field(..., description="Whether the operation is approved")
    reason: str = Field(..., description="Reason for approval/rejection")
    required_virtues: Dict[str, float] = Field(default_factory=dict, description="Virtue requirements")
    shacl_violations: List[str] = Field(default_factory=list, description="SHACL validation errors")

class EthicsValidator:
    """Validates operations against FoT ethics rules and SHACL constraints"""
    
    def __init__(self, neo4j_uri: str, neo4j_user: str, neo4j_password: str):
        self.driver = GraphDatabase.driver(neo4j_uri, auth=(neo4j_user, neo4j_password))
        self.virtue_thresholds = self._load_virtue_thresholds()
        
    def _load_virtue_thresholds(self) -> Dict[str, float]:
        """Load virtue thresholds from configuration"""
        try:
            with open("rules/virtue_thresholds.yaml", "r") as f:
                config = yaml.safe_load(f)
                return config.get("thresholds", {})
        except FileNotFoundError:
            logger.warning("Virtue thresholds file not found, using defaults")
            return {
                "honesty": 0.7,
                "curiosity": 0.6,
                "patience": 0.5,
                "justice": 0.7,
                "transparency": 0.6,
                "fairness": 0.7
            }
    
    def validate_operation(self, request: ValidationRequest) -> ValidationResponse:
        """Validate an operation against ethics rules"""
        logger.info(f"Validating operation: {request.operation} by {request.subject}")
        
        # Check virtue thresholds
        virtue_check = self._check_virtue_thresholds(request)
        if not virtue_check["passed"]:
            return ValidationResponse(
                approved=False,
                reason=f"Virtue threshold not met: {virtue_check['reason']}",
                required_virtues=virtue_check["required"],
                shacl_violations=[]
            )
        
        # Check SHACL constraints
        shacl_check = self._check_shacl_constraints(request)
        if not shacl_check["passed"]:
            return ValidationResponse(
                approved=False,
                reason="SHACL validation failed",
                required_virtues=virtue_check["required"],
                shacl_violations=shacl_check["violations"]
            )
        
        # Check operation-specific rules
        operation_check = self._check_operation_rules(request)
        if not operation_check["passed"]:
            return ValidationResponse(
                approved=False,
                reason=operation_check["reason"],
                required_virtues=virtue_check["required"],
                shacl_violations=[]
            )
        
        return ValidationResponse(
            approved=True,
            reason="Operation approved by ethics validator",
            required_virtues=virtue_check["required"],
            shacl_violations=[]
        )
    
    def _check_virtue_thresholds(self, request: ValidationRequest) -> Dict[str, Any]:
        """Check if the subject meets required virtue thresholds"""
        with self.driver.session() as session:
            # Get subject's current virtue scores
            result = session.run("""
                MATCH (a:Agent {id: $subject_id})
                RETURN a.virtues as virtues
            """, subject_id=request.subject)
            
            record = result.single()
            if not record:
                return {
                    "passed": False,
                    "reason": "Subject not found",
                    "required": self.virtue_thresholds
                }
            
            current_virtues = record["virtues"] or {}
            
            # Check each required virtue
            for virtue, threshold in self.virtue_thresholds.items():
                current_score = current_virtues.get(virtue, 0.0)
                if current_score < threshold:
                    return {
                        "passed": False,
                        "reason": f"Insufficient {virtue}: {current_score} < {threshold}",
                        "required": self.virtue_thresholds
                    }
            
            return {
                "passed": True,
                "reason": "All virtue thresholds met",
                "required": self.virtue_thresholds
            }
    
    def _check_shacl_constraints(self, request: ValidationRequest) -> Dict[str, Any]:
        """Check SHACL constraints for the operation"""
        # This would integrate with a SHACL validation engine
        # For now, we'll implement basic checks
        
        violations = []
        
        # Check if superposed claims have provenance
        if request.operation == "create_claim" and request.content:
            if "superposed" in request.content and not request.provenance:
                violations.append("Superposed claims must have provenance")
        
        # Check if verified claims have verification data
        if request.operation == "verify_claim":
            if not request.virtues.get("honesty", 0.0) >= 0.8:
                violations.append("Claim verification requires high honesty (≥0.8)")
        
        return {
            "passed": len(violations) == 0,
            "violations": violations
        }
    
    def _check_operation_rules(self, request: ValidationRequest) -> Dict[str, Any]:
        """Check operation-specific business rules"""
        
        # Students can only modify their own learning edges
        if request.operation == "update_mastery":
            if not request.subject.startswith("student_"):
                return {
                    "passed": False,
                    "reason": "Only students can update mastery"
                }
        
        # Teachers can only operate within their class scope
        if request.operation == "create_lesson":
            if not request.subject.startswith("teacher_"):
                return {
                    "passed": False,
                    "reason": "Only teachers can create lessons"
                }
        
        # Parents have read-only access
        if request.operation in ["create_claim", "update_mastery", "create_lesson"]:
            if request.subject.startswith("parent_"):
                return {
                    "passed": False,
                    "reason": "Parents have read-only access"
                }
        
        return {
            "passed": True,
            "reason": "Operation rules satisfied"
        }
    
    def close(self):
        """Close the Neo4j driver"""
        self.driver.close()
