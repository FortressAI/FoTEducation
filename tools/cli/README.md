# FoT CLI Tool

Command-line interface for managing the FoT Knowledge-Miner Platform.

## Installation

```bash
# Install dependencies
pip install -r requirements.txt

# Make executable
chmod +x fot-cli.py
```

## Usage

### Initialize Database
```bash
python fot-cli.py init-db
```

### Load Seed Data
```bash
# Load default seeds
python fot-cli.py load-seeds

# Load specific files
python fot-cli.py load-seeds graph/seeds/biology.ttl graph/seeds/virtues.ttl
```

### Create Class
```bash
python fot-cli.py create-class "Biology 101" teacher_123
```

### Add Users
```bash
# Add student
python fot-cli.py add-user student student_123 "John Doe" --class-id class_biology_101

# Add teacher
python fot-cli.py add-user teacher teacher_123 "Dr. Smith"

# Add parent
python fot-cli.py add-user parent parent_123 "Jane Doe"
```

## Configuration

Set environment variables or use command-line options:
- `--neo4j-uri`: Neo4j connection URI (default: bolt://localhost:7687)
- `--neo4j-user`: Neo4j username (default: neo4j)
- `--neo4j-password`: Neo4j password (default: neo4jpassword)

## Examples

```bash
# Full setup workflow
python fot-cli.py init-db
python fot-cli.py load-seeds
python fot-cli.py add-user teacher teacher_demo "Demo Teacher"
python fot-cli.py create-class "Physics 101" teacher_demo
python fot-cli.py add-user student student_demo "Demo Student" --class-id class_physics_101
```
