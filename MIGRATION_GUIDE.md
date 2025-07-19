# Repository Reorganization Migration Guide

This document outlines the structural changes made to the Free Deep Research System repository and provides guidance for adapting to the new organization.

## 📋 Summary of Changes

The repository has been reorganized from a flat, mixed-content structure to a professional, modular architecture following industry best practices.

### Before (Old Structure)
```
free-deep-research/
├── [20+ documentation files in root]
├── ai-orchestrator/
├── bmad-agent/
│   ├── deep_research_frontend/
│   └── free-deep-research/
├── docker/
├── scripts/
└── [various config files]
```

### After (New Structure)
```
free-deep-research/
├── apps/                           # Applications
│   ├── desktop/                    # Main Tauri app
│   └── web/                        # Web frontend
├── packages/                       # Shared packages
│   ├── ai-orchestrator/            # AI orchestration
│   └── bmad-core/                  # BMAD configurations
├── docs/                           # All documentation
│   ├── api/
│   ├── architecture/
│   ├── deployment/
│   ├── development/
│   ├── reports/
│   └── user-guides/
├── infrastructure/                 # Infrastructure & deployment
│   ├── docker/
│   ├── scripts/
│   └── docker-compose files
└── tools/                          # Development tools
```

## 🔄 File Migrations

### Applications
| Old Location | New Location | Notes |
|--------------|--------------|-------|
| `bmad-agent/free-deep-research/` | `apps/desktop/` | Main Tauri desktop application |
| `bmad-agent/deep_research_frontend/` | `apps/web/` | React web application |

### Packages
| Old Location | New Location | Notes |
|--------------|--------------|-------|
| `ai-orchestrator/` | `packages/ai-orchestrator/` | AI orchestration system |
| `bmad-agent/[configs]` | `packages/bmad-core/` | BMAD agent configurations |

### Documentation
| Old Location | New Location | Category |
|--------------|--------------|----------|
| `COMPLETE_USER_GUIDE_2025.md` | `docs/user-guides/` | User documentation |
| `DOCKER-IMPLEMENTATION-SUMMARY.md` | `docs/deployment/` | Deployment guides |
| `TECHNICAL_ISSUES_ANALYSIS.md` | `docs/development/` | Development docs |
| `*GAP_ANALYSIS*.md` | `docs/reports/` | Analysis reports |
| `*PHASE_*.md` | `docs/reports/` | Project reports |

### Infrastructure
| Old Location | New Location | Notes |
|--------------|--------------|-------|
| `docker/` | `infrastructure/docker/` | Docker configurations |
| `scripts/` | `infrastructure/scripts/` | Build and deployment scripts |
| `docker-compose*.yml` | `infrastructure/` | Compose configurations |
| `setup.sh`, `setup.bat` | `infrastructure/scripts/` | Setup scripts |

## 🛠️ Required Updates

### 1. Development Workflow Changes

#### Old Workflow
```bash
# Old way
cd free-deep-research
npm run dev
```

#### New Workflow
```bash
# Desktop app
cd apps/desktop
npm run dev

# Web app
cd apps/web
npm run dev

# Full system
docker-compose -f infrastructure/docker-compose.dev.yml up
```

### 2. Build Script Updates

#### Package.json Updates
The desktop app's `package.json` has been updated:
- Repository directory: `bmad-agent/free-deep-research` → `apps/desktop`

#### Docker Compose Updates
```bash
# Old
docker-compose up

# New
docker-compose -f infrastructure/docker-compose.yml up
```

### 3. Import Path Updates

If you have any custom scripts or configurations that reference the old paths, update them:

```bash
# Old paths
bmad-agent/free-deep-research/src/
ai-orchestrator/core/

# New paths  
apps/desktop/src/
packages/ai-orchestrator/core/
```

### 4. Documentation References

Update any bookmarks or references to documentation:
- User guides: `docs/user-guides/`
- API docs: `docs/api/`
- Deployment: `docs/deployment/`
- Development: `docs/development/`

## 🔧 Setup Instructions

### For Existing Developers

1. **Pull the latest changes**
   ```bash
   git pull origin main
   ```

2. **Clean old dependencies**
   ```bash
   # Remove old node_modules if they exist
   rm -rf node_modules
   rm -rf bmad-agent/*/node_modules
   ```

3. **Install dependencies in new locations**
   ```bash
   # Desktop app
   cd apps/desktop
   npm install
   
   # Web app
   cd apps/web
   npm install
   ```

4. **Update your development environment**
   ```bash
   # Use new setup script
   ./infrastructure/scripts/setup.sh
   ```

### For New Developers

1. **Clone and setup**
   ```bash
   git clone https://github.com/usemanusai/free-deep-research.git
   cd free-deep-research
   ./infrastructure/scripts/setup.sh
   ```

2. **Choose your development path**
   ```bash
   # Desktop development
   cd apps/desktop
   npm run dev
   
   # Web development
   cd apps/web
   npm run dev
   
   # Full stack development
   docker-compose -f infrastructure/docker-compose.dev.yml up
   ```

## 📚 Updated Documentation

### New README Structure
- **Main README**: Overview and quick start
- **Apps README**: Application-specific documentation
- **Packages README**: Shared package documentation
- **Infrastructure README**: Deployment and operations
- **Docs README**: Documentation index

### Key Documentation Files
- [Main README](README.md) - Repository overview
- [User Guide](docs/user-guides/COMPLETE_USER_GUIDE_2025.md) - End-user documentation
- [Docker Deployment](docs/deployment/DOCKER-IMPLEMENTATION-SUMMARY.md) - Deployment guide
- [Development Guide](docs/development/) - Developer documentation

## 🚨 Breaking Changes

### 1. File Paths
All file paths have changed. Update any:
- Import statements
- Configuration files
- Build scripts
- Documentation links

### 2. Docker Compose Commands
```bash
# Old
docker-compose up

# New
docker-compose -f infrastructure/docker-compose.yml up
```

### 3. Setup Scripts
```bash
# Old
./setup.sh

# New
./infrastructure/scripts/setup.sh
```

## ✅ Benefits of New Structure

### 1. **Clear Separation of Concerns**
- Applications in `apps/`
- Shared code in `packages/`
- Documentation in `docs/`
- Infrastructure in `infrastructure/`

### 2. **Improved Maintainability**
- Easier to find relevant files
- Logical grouping of related components
- Consistent naming conventions

### 3. **Better Developer Experience**
- Clear entry points for different roles
- Comprehensive documentation structure
- Professional repository organization

### 4. **Scalability**
- Easy to add new applications
- Modular package structure
- Organized documentation growth

## 🤝 Support

If you encounter issues with the migration:

1. **Check this migration guide** for common scenarios
2. **Review the updated README files** in each directory
3. **Open an issue** if you find broken functionality
4. **Ask for help** in discussions for setup questions

## 📝 Changelog

- **2025-07-19**: Complete repository reorganization
  - Moved applications to `apps/` directory
  - Organized shared packages in `packages/`
  - Consolidated documentation in `docs/`
  - Moved infrastructure to `infrastructure/`
  - Updated all README files and documentation
  - Created comprehensive migration guide

---

*This migration maintains all functionality while providing a professional, maintainable repository structure.*
