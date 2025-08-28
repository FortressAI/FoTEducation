-- FoT Knowledge-Miner Platform Neo4j Schema
-- This file contains all constraints, indexes, and schema definitions

-- Create constraints for unique identifiers
CREATE CONSTRAINT concept_id_unique IF NOT EXISTS FOR (c:Concept) REQUIRE c.id IS UNIQUE;
CREATE CONSTRAINT claim_id_unique IF NOT EXISTS FOR (c:Claim) REQUIRE c.id IS UNIQUE;
CREATE CONSTRAINT agent_id_unique IF NOT EXISTS FOR (a:Agent) REQUIRE a.id IS UNIQUE;
CREATE CONSTRAINT virtue_id_unique IF NOT EXISTS FOR (v:Virtue) REQUIRE v.id IS UNIQUE;

-- Create constraints for required properties
CREATE CONSTRAINT concept_label_exists IF NOT EXISTS FOR (c:Concept) REQUIRE c.label IS NOT NULL;
CREATE CONSTRAINT claim_state_exists IF NOT EXISTS FOR (c:Claim) REQUIRE c.state IS NOT NULL;
CREATE CONSTRAINT claim_confidence_range IF NOT EXISTS FOR (c:Claim) REQUIRE c.confidence >= 0.0 AND c.confidence <= 1.0;

-- Create indexes for performance
CREATE INDEX concept_domain_idx IF NOT EXISTS FOR (c:Concept) ON (c.domain);
CREATE INDEX claim_state_idx IF NOT EXISTS FOR (c:Claim) ON (c.state);
CREATE INDEX claim_confidence_idx IF NOT EXISTS FOR (c:Claim) ON (c.confidence);
CREATE INDEX agent_role_idx IF NOT EXISTS FOR (a:Agent) ON (a.role);

-- Create indexes for relationship properties
CREATE INDEX learns_mastery_idx IF NOT EXISTS FOR ()-[r:LEARNS]-() ON (r.mastery);
CREATE INDEX learns_curiosity_idx IF NOT EXISTS FOR ()-[r:LEARNS]-() ON (r.curiosity);
CREATE INDEX learns_patience_idx IF NOT EXISTS FOR ()-[r:LEARNS]-() ON (r.patience);
CREATE INDEX learns_honesty_idx IF NOT EXISTS FOR ()-[r:LEARNS]-() ON (r.honesty);

-- Create fulltext indexes for search
CREATE FULLTEXT INDEX concept_search_idx IF NOT EXISTS FOR (c:Concept) ON EACH [c.label, c.description];
CREATE FULLTEXT INDEX claim_search_idx IF NOT EXISTS FOR (c:Claim) ON EACH [c.content, c.summary];

-- Create composite indexes for common query patterns
CREATE INDEX concept_domain_label_idx IF NOT EXISTS FOR (c:Concept) ON (c.domain, c.label);
CREATE INDEX claim_state_confidence_idx IF NOT EXISTS FOR (c:Claim) ON (c.state, c.confidence);
CREATE INDEX agent_role_class_idx IF NOT EXISTS FOR (a:Agent) ON (a.role, a.class_id);
