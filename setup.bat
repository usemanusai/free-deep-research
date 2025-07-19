@echo off
setlocal enabledelayedexpansion

REM Free Deep Research System - Docker Setup Script for Windows
REM Version 3.0.0 "Global Intelligence Network"
REM Compatible with: Windows 10/11, Windows Server 2019/2022

REM Configuration
set "PROJECT_NAME=free-deep-research"
set "VERSION=3.0.0"
set "SCRIPT_DIR=%~dp0"

REM Default values
set "ENVIRONMENT=development"
set "SKIP_DEPS=false"
set "SKIP_SSL=false"
set "SKIP_MIGRATION=false"
set "VERBOSE=false"

REM Colors for output (Windows 10+ with ANSI support)
set "RED=[31m"
set "GREEN=[32m"
set "YELLOW=[33m"
set "BLUE=[34m"
set "PURPLE=[35m"
set "CYAN=[36m"
set "NC=[0m"

REM Function to print colored output
:print_status
echo %BLUE%[INFO]%NC% %~1
goto :eof

:print_success
echo %GREEN%[SUCCESS]%NC% %~1
goto :eof

:print_warning
echo %YELLOW%[WARNING]%NC% %~1
goto :eof

:print_error
echo %RED%[ERROR]%NC% %~1
goto :eof

:print_header
echo %PURPLE%%~1%NC%
goto :eof

REM Function to show usage
:show_usage
echo Free Deep Research System - Docker Setup Script v%VERSION%
echo.
echo Usage: %~nx0 [OPTIONS]
echo.
echo OPTIONS:
echo     -e, --environment ENV    Set environment (development^|production) [default: development]
echo     -s, --skip-deps         Skip dependency checks
echo     --skip-ssl              Skip SSL certificate generation
echo     --skip-migration        Skip database migration
echo     -v, --verbose           Enable verbose output
echo     -h, --help              Show this help message
echo.
echo EXAMPLES:
echo     %~nx0                      # Setup development environment
echo     %~nx0 -e production        # Setup production environment
echo     %~nx0 --skip-deps -v       # Skip dependency checks with verbose output
echo.
echo ENVIRONMENT VARIABLES:
echo     FDR_ENVIRONMENT         Override environment setting
echo     FDR_SKIP_DEPS          Skip dependency checks (true/false)
echo     FDR_VERBOSE            Enable verbose output (true/false)
echo.
goto :eof

REM Parse command line arguments
:parse_args
if "%~1"=="" goto :parse_args_done
if "%~1"=="-e" (
    set "ENVIRONMENT=%~2"
    shift
    shift
    goto :parse_args
)
if "%~1"=="--environment" (
    set "ENVIRONMENT=%~2"
    shift
    shift
    goto :parse_args
)
if "%~1"=="-s" (
    set "SKIP_DEPS=true"
    shift
    goto :parse_args
)
if "%~1"=="--skip-deps" (
    set "SKIP_DEPS=true"
    shift
    goto :parse_args
)
if "%~1"=="--skip-ssl" (
    set "SKIP_SSL=true"
    shift
    goto :parse_args
)
if "%~1"=="--skip-migration" (
    set "SKIP_MIGRATION=true"
    shift
    goto :parse_args
)
if "%~1"=="-v" (
    set "VERBOSE=true"
    shift
    goto :parse_args
)
if "%~1"=="--verbose" (
    set "VERBOSE=true"
    shift
    goto :parse_args
)
if "%~1"=="-h" (
    call :show_usage
    exit /b 0
)
if "%~1"=="--help" (
    call :show_usage
    exit /b 0
)
call :print_error "Unknown option: %~1"
call :show_usage
exit /b 1

:parse_args_done
REM Override with environment variables
if defined FDR_ENVIRONMENT set "ENVIRONMENT=%FDR_ENVIRONMENT%"
if defined FDR_SKIP_DEPS set "SKIP_DEPS=%FDR_SKIP_DEPS%"
if defined FDR_VERBOSE set "VERBOSE=%FDR_VERBOSE%"
goto :eof

REM Function to check if command exists
:command_exists
where "%~1" >nul 2>&1
goto :eof

REM Function to check system requirements
:check_requirements
if "%SKIP_DEPS%"=="true" (
    call :print_warning "Skipping dependency checks as requested"
    goto :eof
)

call :print_status "Checking system requirements..."

set "missing_deps="

REM Check Docker
call :command_exists docker
if errorlevel 1 (
    set "missing_deps=!missing_deps! docker"
) else (
    for /f "tokens=3" %%i in ('docker --version 2^>nul') do (
        call :print_status "Found Docker version: %%i"
    )
)

REM Check Docker Compose
call :command_exists docker-compose
if errorlevel 1 (
    docker compose version >nul 2>&1
    if errorlevel 1 (
        set "missing_deps=!missing_deps! docker-compose"
    ) else (
        call :print_status "Found Docker Compose (plugin version)"
    )
) else (
    for /f "tokens=3" %%i in ('docker-compose --version 2^>nul') do (
        call :print_status "Found Docker Compose version: %%i"
    )
)

REM Check curl
call :command_exists curl
if errorlevel 1 (
    set "missing_deps=!missing_deps! curl"
)

REM Check openssl for SSL certificate generation
if "%SKIP_SSL%" neq "true" (
    call :command_exists openssl
    if errorlevel 1 (
        set "missing_deps=!missing_deps! openssl"
    )
)

if defined missing_deps (
    call :print_error "Missing required dependencies:!missing_deps!"
    call :print_status "Please install the missing dependencies and run this script again."
    call :print_status "You can install Docker Desktop from: https://www.docker.com/products/docker-desktop"
    call :print_status "You can install OpenSSL from: https://slproweb.com/products/Win32OpenSSL.html"
    exit /b 1
)

call :print_success "All system requirements satisfied"
goto :eof

REM Function to setup port management
:setup_port_management
call :print_status "Setting up intelligent port management..."

REM Generate port registry
docker\port-manager\port-manager.bat generate "%ENVIRONMENT%"
if errorlevel 1 (
    call :print_error "Failed to generate port registry"
    exit /b 1
)

call :print_success "Port registry generated successfully"
goto :eof

REM Function to setup environment files
:setup_environment
call :print_status "Setting up environment configuration for: %ENVIRONMENT%"

set "env_file=.env"
set "source_env_file="

if "%ENVIRONMENT%"=="development" (
    set "source_env_file=.env.dev"
) else if "%ENVIRONMENT%"=="production" (
    set "source_env_file=.env.prod"
) else (
    call :print_error "Invalid environment: %ENVIRONMENT%. Must be 'development' or 'production'"
    exit /b 1
)

REM Copy environment file
if exist "%source_env_file%" (
    copy "%source_env_file%" "%env_file%" >nul
    call :print_success "Environment file copied from %source_env_file% to %env_file%"
) else (
    call :print_warning "Source environment file %source_env_file% not found, using .env.example"
    if exist ".env.example" (
        copy ".env.example" "%env_file%" >nul
    ) else (
        call :print_error "No environment template found!"
        exit /b 1
    )
)

REM Set build date
for /f "tokens=1-3 delims=/ " %%a in ('date /t') do (
    set "build_date=%%c-%%a-%%b"
)
powershell -Command "(Get-Content '%env_file%') -replace 'BUILD_DATE=.*', 'BUILD_DATE=%build_date%' | Set-Content '%env_file%'"

REM Setup port management
call :setup_port_management

REM Merge port registry with environment file
if exist ".env.ports" (
    call :print_status "Merging port registry with environment file..."
    echo. >> "%env_file%"
    echo # Dynamic Port Assignments (Auto-generated) >> "%env_file%"
    type .env.ports >> "%env_file%"
    call :print_success "Port assignments merged into environment file"
)

call :print_success "Environment configuration completed"
goto :eof

REM Function to create necessary directories
:create_directories
call :print_status "Creating necessary directories..."

set directories=docker\nginx\logs docker\nginx\ssl docker\nginx\cache docker\nginx\html docker\backend\logs docker\backend\uploads docker\backend\cache docker\backend\ssl docker\frontend\logs docker\database\backups docker\database\init docker\redis docker\prometheus docker\grafana\provisioning\dashboards docker\grafana\provisioning\datasources docker\grafana\dashboards docker\loki docker\backup bmad-agent\free-deep-research\data

for %%d in (%directories%) do (
    if not exist "%%d" (
        mkdir "%%d" 2>nul
        if "%VERBOSE%"=="true" (
            call :print_status "Created directory: %%d"
        )
    )
)

call :print_success "Directory structure created"
goto :eof

REM Function to generate SSL certificates
:generate_ssl_certificates
if "%SKIP_SSL%"=="true" (
    call :print_warning "Skipping SSL certificate generation as requested"
    goto :eof
)

call :print_status "Generating SSL certificates..."

set "ssl_dir=docker\nginx\ssl"
set "cert_file=%ssl_dir%\cert.pem"
set "key_file=%ssl_dir%\key.pem"
set "dhparam_file=%ssl_dir%\dhparam.pem"

REM Generate self-signed certificate
if "%ENVIRONMENT%"=="development" (
    openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout "%key_file%" -out "%cert_file%" -subj "/C=US/ST=Development/L=Development/O=Free Deep Research/OU=Development/CN=localhost" >nul 2>&1
    call :print_success "Self-signed SSL certificate generated for development"
) else (
    call :print_warning "Production environment detected. Please replace the generated certificates with valid ones."
    openssl req -x509 -nodes -days 30 -newkey rsa:2048 -keyout "%key_file%" -out "%cert_file%" -subj "/C=US/ST=Production/L=Production/O=Free Deep Research/OU=Production/CN=your-domain.com" >nul 2>&1
)

REM Generate Diffie-Hellman parameters
if not exist "%dhparam_file%" (
    call :print_status "Generating Diffie-Hellman parameters (this may take a while)..."
    openssl dhparam -out "%dhparam_file%" 2048 >nul 2>&1
    call :print_success "Diffie-Hellman parameters generated"
)
goto :eof

REM Function to build and start services
:start_services
call :print_status "Building and starting services for %ENVIRONMENT% environment..."

set "compose_file="
if "%ENVIRONMENT%"=="development" (
    set "compose_file=docker-compose.dev.yml"
) else (
    set "compose_file=docker-compose.prod.yml"
)

REM Build images
call :print_status "Building Docker images..."
if "%VERBOSE%"=="true" (
    docker-compose -f "%compose_file%" build
) else (
    docker-compose -f "%compose_file%" build >nul 2>&1
)

REM Start services
call :print_status "Starting services..."
if "%VERBOSE%"=="true" (
    docker-compose -f "%compose_file%" up -d
) else (
    docker-compose -f "%compose_file%" up -d >nul 2>&1
)

call :print_success "Services started successfully"
goto :eof

REM Function to show final status
:show_final_status
call :print_header "╔══════════════════════════════════════════════════════════════════════════════╗"
call :print_header "║                    Free Deep Research System v%VERSION%                     ║"
call :print_header "║                        'Global Intelligence Network'                        ║"
call :print_header "║                            Setup Complete!                                  ║"
call :print_header "╚══════════════════════════════════════════════════════════════════════════════╝"

call :print_success "Environment: %ENVIRONMENT%"
call :print_success "All services are running successfully!"

echo.
call :print_status "Service URLs (Dynamic Port Assignments):"

REM Display dynamically assigned ports
docker\port-manager\port-manager.bat status
if errorlevel 1 (
    call :print_warning "Could not display port assignments, using defaults:"

    if "%ENVIRONMENT%"=="development" (
        echo   🌐 Frontend:           http://localhost:3000
        echo   🔧 Backend API:        http://localhost:8080
        echo   📊 Prometheus:         http://localhost:9090
        echo   📈 Grafana:            http://localhost:3001
        echo   🗄️  Database Admin:     http://localhost:8082
        echo   🔴 Redis Commander:    http://localhost:8083
        echo   📧 Mailhog:            http://localhost:8025
        echo   🛠️  Dev Dashboard:      http://localhost:8081
    ) else (
        echo   🌐 Frontend:           https://localhost
        echo   🔧 Backend API:        https://localhost/api
        echo   📊 Prometheus:         http://localhost:9090
        echo   📈 Grafana:            http://localhost:3001
    )
) else (
    call :print_success "Port assignments displayed above"
)

echo.
call :print_status "Useful commands:"
echo   📋 View logs:          docker-compose logs -f
echo   🔄 Restart services:   docker-compose restart
echo   🛑 Stop services:      docker-compose down
echo   🧹 Clean up:           docker-compose down -v --remove-orphans
echo.
call :print_status "Port management commands:"
echo   🔍 Check port status:  docker\port-manager\port-manager.bat status
echo   🔄 Regenerate ports:   docker\port-manager\port-manager.bat regenerate %ENVIRONMENT%
echo   🧹 Cleanup ports:      docker\port-manager\port-manager.bat cleanup

if "%ENVIRONMENT%"=="production" (
    echo.
    call :print_warning "PRODUCTION SETUP REMINDERS:"
    echo   1. Update all passwords in .env file
    echo   2. Replace SSL certificates with valid ones
    echo   3. Configure proper domain names
    echo   4. Set up proper backup procedures
    echo   5. Configure monitoring and alerting
)

echo.
call :print_status "For more information, see README-Docker.md"
goto :eof

REM Main execution
:main
call :print_header "╔══════════════════════════════════════════════════════════════════════════════╗"
call :print_header "║                    Free Deep Research System v%VERSION%                     ║"
call :print_header "║                        'Global Intelligence Network'                        ║"
call :print_header "║                           Docker Setup Script                               ║"
call :print_header "╚══════════════════════════════════════════════════════════════════════════════╝"

call :parse_args %*

call :print_status "Starting setup for %ENVIRONMENT% environment..."

call :check_requirements
if errorlevel 1 exit /b 1

call :setup_environment
if errorlevel 1 exit /b 1

call :create_directories
if errorlevel 1 exit /b 1

call :generate_ssl_certificates
if errorlevel 1 exit /b 1

call :start_services
if errorlevel 1 exit /b 1

call :show_final_status

goto :eof

REM Run main function
call :main %*
