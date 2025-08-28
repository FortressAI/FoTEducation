# FoT Knowledge-Miner Platform - Local Development Runbook

## 🚀 **Quick Start (90 minutes)**

This guide will get you from zero to a fully running FoT Knowledge-Miner Platform on your local machine.

## 📋 **Prerequisites**

- **OS**: macOS, Linux, or Windows with WSL2
- **RAM**: Minimum 16GB, recommended 32GB+
- **Storage**: 50GB+ free space
- **Docker**: Docker Desktop or Docker Engine
- **Python**: 3.9+ with pip
- **Node.js**: 18+ with npm/pnpm

## 🏗 **Step 1: Repository Setup**

```bash
# Clone the repository
git clone https://github.com/FortressAI/FoTEducation.git
cd FoTEducation

# Verify the structure
ls -la
# Should show: apps/, agents/, services/, graph/, deploy/, tools/, tests/, docs/
```

## 🔧 **Step 2: Environment Configuration**

```bash
# Copy environment template
cp env.example .env

# Edit environment variables (use your preferred editor)
nano .env
# or
code .env
```

**Key settings to configure:**
```bash
# Database
NEO4J_AUTH=neo4j/your-secure-password
NEO4J_URI=bolt://localhost:7687

# JWT (generate a secure random key)
JWT_SECRET=your-super-secure-jwt-key-here

# API
API_PORT=8080
API_HOST=0.0.0.0
```

## 🐳 **Step 3: Start Core Services**

```bash
# Navigate to deploy directory
cd deploy

# Start the core services
docker compose up -d

# Verify services are running
docker compose ps
```

**Expected output:**
```
NAME                COMMAND                  SERVICE             STATUS              PORTS
fot-ethics-1       "python -m uvicorn …"   ethics              running             
fot-miners-wiki-1  "python main.py"        miners-wiki         running             
fot-neo4j-1        "tini -g -- /startup…"   neo4j               running             0.0.0.0:7474->7474/tcp, 0.0.0.0:7687->7687/tcp
fot-qdrant-1       "/qdrant"                qdrant              running             0.0.0.0:6333->6333/tcp
fot-tutor-1        "python -m uvicorn …"    tutor               running             
```

## 🗄 **Step 4: Initialize Knowledge Graph**

```bash
# Navigate back to project root
cd ..

# Install Python dependencies
pip install -r services/ethics-measurement/requirements.txt

# Initialize the database
python tools/cli/fot-cli.py init-db

# Load seed data
python tools/cli/fot-cli.py load-seeds

# Create a demo teacher
python tools/cli/fot-cli.py add-user teacher teacher_demo "Demo Teacher"

# Create a demo class
python tools/cli/fot-cli.py create-class "Biology 101" teacher_demo

# Create a demo student
python tools/cli/fot-cli.py add-user student student_demo "Demo Student" --class-id class_biology_101

# Create a demo parent
python tools/cli/fot-cli.py add-user parent parent_demo "Demo Parent"
```

## 🌐 **Step 5: Start Web Applications**

```bash
# Install Node.js dependencies for student web app
cd apps/student-web
npm install
# or
pnpm install

# Start development server
npm run dev
# or
pnpm dev

# In a new terminal, start teacher web app
cd ../teacher-web
npm install
npm run dev

# In another terminal, start parent web app
cd ../parent-web
npm install
npm run dev
```

## 🧪 **Step 6: Verify Everything Works**

### **Check Neo4j Browser**
1. Open http://localhost:7474
2. Login with `neo4j` / `your-password`
3. Run: `MATCH (n) RETURN n LIMIT 10`

### **Check API Gateway**
1. Open http://localhost:8080/docs
2. Should see FastAPI documentation

### **Check Web Apps**
1. Student: http://localhost:5173 (or port shown in terminal)
2. Teacher: http://localhost:5174 (or port shown in terminal)
3. Parent: http://localhost:5175 (or port shown in terminal)

## 🎯 **Step 7: First Demo (Photosynthesis Lesson)**

### **Teacher Interface**
1. Login as `teacher_demo`
2. Navigate to "Create Lesson"
3. Select "Biology" domain
4. Choose "Photosynthesis" concept
5. Create lesson content
6. Assign to "Biology 101" class

### **Student Interface**
1. Login as `student_demo`
2. See assigned "Photosynthesis" lesson
3. Complete the lesson module
4. Watch virtue metrics update (curiosity +0.1, patience +0.05)

### **Parent Interface**
1. Login as `parent_demo`
2. View child's progress
3. See virtue development trends
4. Monitor cross-domain connections

## 🔍 **Troubleshooting**

### **Service Won't Start**
```bash
# Check logs
docker compose logs [service-name]

# Common issues:
# - Port conflicts: Change ports in docker-compose.yml
# - Memory issues: Increase Docker memory limit
# - Permission issues: Check file permissions
```

### **Database Connection Issues**
```bash
# Test Neo4j connection
python -c "
from neo4j import GraphDatabase
driver = GraphDatabase.driver('bolt://localhost:7687', auth=('neo4j', 'your-password'))
with driver.session() as session:
    result = session.run('RETURN 1 as test')
    print(result.single())
driver.close()
"
```

### **Web App Build Issues**
```bash
# Clear node_modules and reinstall
rm -rf node_modules package-lock.json
npm install

# Check Node.js version
node --version  # Should be 18+
```

## 📊 **Monitoring & Debugging**

### **Service Health Checks**
```bash
# Check all services
docker compose ps

# View real-time logs
docker compose logs -f

# Check resource usage
docker stats
```

### **Database Queries**
```bash
# Connect to Neo4j shell
docker exec -it fot-neo4j-1 cypher-shell -u neo4j -p your-password

# Useful queries:
# View all concepts
MATCH (c:Concept) RETURN c;

# View student progress
MATCH (s:Student)-[r:LEARNS]->(c:Concept) RETURN s.name, c.label, r.mastery;

# View virtue development
MATCH (a:Agent) RETURN a.id, a.virtues;
```

## 🚀 **Next Steps**

### **Development Workflow**
1. **Modify WASM agents**: Edit Rust code in `agents/`
2. **Update ontologies**: Modify OWL/RDF files in `graph/ontologies/`
3. **Add new domains**: Create new topic agents and seed data
4. **Extend ethics rules**: Modify validation logic in `services/ethics-measurement/`

### **Testing**
```bash
# Run integration tests
cd tests
npm test

# Run specific test suites
npm run test:e2e
npm run test:agents
```

### **Production Deployment**
- See `docs/RUNBOOK_K8S.md` for Kubernetes deployment
- See `docs/ARCHITECTURE.md` for scaling considerations

## 🎉 **Success Indicators**

You've successfully set up the FoT platform when:
- ✅ All Docker services are running
- ✅ Neo4j contains seed data (concepts, virtues, claims)
- ✅ Web apps load without errors
- ✅ Teacher can create a lesson
- ✅ Student can complete a lesson
- ✅ Parent can view progress
- ✅ Virtue metrics update in real-time
- ✅ Cross-domain connections are visible

## 🆘 **Getting Help**

- **Issues**: Create GitHub issue with logs and error messages
- **Documentation**: Check `docs/` directory for detailed guides
- **Community**: Join discussions in repository discussions
- **Architecture**: Review `docs/ARCHITECTURE.md` for system design

---

**Welcome to the quantum future of education! 🚀**
