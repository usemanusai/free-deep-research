@echo off
setlocal enabledelayedexpansion

REM Free Deep Research System - Intelligent Port Manager (Windows)
REM Handles dynamic port allocation, conflict prevention, and lifecycle management

REM Configuration
set "SCRIPT_DIR=%~dp0"
set "PROJECT_ROOT=%SCRIPT_DIR%..\.."
set "PORT_REGISTRY_FILE=%PROJECT_ROOT%\.env.ports"
set "PORT_LOCK_DIR=%TEMP%\fdr-port-locks"
set "PROJECT_NAME=free-deep-research"

REM Colors for output (Windows 10+ with ANSI support)
set "RED=[31m"
set "GREEN=[32m"
set "YELLOW=[33m"
set "BLUE=[34m"
set "PURPLE=[35m"
set "NC=[0m"

REM Function to print colored output
:print_status
echo %BLUE%[PORT-MGR]%NC% %~1
goto :eof

:print_success
echo %GREEN%[PORT-MGR]%NC% %~1
goto :eof

:print_warning
echo %YELLOW%[PORT-MGR]%NC% %~1
goto :eof

:print_error
echo %RED%[PORT-MGR]%NC% %~1
goto :eof

:print_header
echo %PURPLE%%~1%NC%
goto :eof

REM Function to create lock directory
:create_lock_dir
if not exist "%PORT_LOCK_DIR%" mkdir "%PORT_LOCK_DIR%"
goto :eof

REM Function to check if port is available
:is_port_available
set "port=%~1"
set "available=1"

REM Check if port is in use using netstat
for /f "tokens=2" %%a in ('netstat -an ^| findstr ":%port% "') do (
    set "available=0"
)

REM Return result via environment variable
set "PORT_AVAILABLE=%available%"
goto :eof

REM Function to get random port from range
:get_random_port_from_range
set "range=%~1"
for /f "tokens=1,2 delims=-" %%a in ("%range%") do (
    set "min_port=%%a"
    set "max_port=%%b"
)

set /a "port_range=%max_port% - %min_port% + 1"
set /a "random_port=%min_port% + %RANDOM% %% %port_range%"
set "RANDOM_PORT=%random_port%"
goto :eof

REM Function to allocate port with conflict prevention
:allocate_port
set "service_type=%~1"
set "service_name=%~2"
set "max_attempts=50"
set "attempt=1"

REM Define port ranges
if "%service_type%"=="frontend" set "range=30000-35000"
if "%service_type%"=="backend" set "range=35000-40000"
if "%service_type%"=="database" set "range=40000-45000"
if "%service_type%"=="redis" set "range=45000-50000"
if "%service_type%"=="nginx" set "range=50000-55000"
if "%service_type%"=="monitoring" set "range=55000-60000"
if "%service_type%"=="devtools" set "range=60000-65000"

if not defined range (
    call :print_error "Unknown service type: %service_type%"
    set "ALLOCATED_PORT="
    goto :eof
)

call :print_status "Allocating port for %service_name% (type: %service_type%, range: %range%)"

:allocation_loop
if %attempt% gtr %max_attempts% (
    call :print_error "Failed to allocate port for %service_name% after %max_attempts% attempts"
    set "ALLOCATED_PORT="
    goto :eof
)

call :get_random_port_from_range "%range%"
set "port=%RANDOM_PORT%"
set "lock_file=%PORT_LOCK_DIR%\%port%.lock"

REM Check if port is locked by our system
if exist "%lock_file%" (
    set /a "attempt+=1"
    goto :allocation_loop
)

REM Check if port is available system-wide
call :is_port_available "%port%"
if "%PORT_AVAILABLE%"=="1" (
    REM Create lock file
    echo %service_name%:%date%:%time%:%RANDOM% > "%lock_file%"
    
    REM Double-check port is still available after lock
    call :is_port_available "%port%"
    if "%PORT_AVAILABLE%"=="1" (
        call :print_success "Allocated port %port% for %service_name%"
        set "ALLOCATED_PORT=%port%"
        goto :eof
    ) else (
        REM Remove lock if port became unavailable
        if exist "%lock_file%" del "%lock_file%"
    )
)

set /a "attempt+=1"

REM Simple delay for backoff
if %attempt% gtr 10 (
    timeout /t 1 /nobreak >nul
)

goto :allocation_loop

REM Function to cleanup orphaned locks
:cleanup_orphaned_locks
call :print_status "Cleaning up orphaned port locks..."

if not exist "%PORT_LOCK_DIR%" goto :eof

set "cleaned=0"
for %%f in ("%PORT_LOCK_DIR%\*.lock") do (
    if exist "%%f" (
        del "%%f"
        set /a "cleaned+=1"
    )
)

if %cleaned% gtr 0 (
    call :print_success "Cleaned up %cleaned% orphaned port locks"
) else (
    call :print_status "No orphaned port locks found"
)
goto :eof

REM Function to scan existing containers
:scan_existing_containers
call :print_status "Scanning for existing containers..."

set "existing_containers="
if exist "%DOCKER_COMMAND%" (
    for /f "tokens=*" %%a in ('docker ps -a --filter "name=%PROJECT_NAME%" --format "{{.Names}}" 2^>nul') do (
        set "existing_containers=!existing_containers! %%a"
        echo   - %%a
    )
)

if defined existing_containers (
    call :print_warning "Found existing containers: %existing_containers%"
) else (
    call :print_status "No existing containers found"
)
goto :eof

REM Function to generate port registry
:generate_port_registry
set "environment=%~1"
if not defined environment set "environment=development"

call :print_header "Generating port registry for %environment% environment..."

REM Create lock directory
call :create_lock_dir

REM Cleanup orphaned locks
call :cleanup_orphaned_locks

REM Check for existing containers
call :scan_existing_containers

REM Create port registry file
echo # Free Deep Research System - Port Registry > "%PORT_REGISTRY_FILE%"
echo # Generated on: %date% %time% >> "%PORT_REGISTRY_FILE%"
echo # Environment: %environment% >> "%PORT_REGISTRY_FILE%"
echo # Project: %PROJECT_NAME% >> "%PORT_REGISTRY_FILE%"
echo. >> "%PORT_REGISTRY_FILE%"
echo # WARNING: This file is auto-generated. Do not edit manually. >> "%PORT_REGISTRY_FILE%"
echo # Use the port manager to modify port assignments. >> "%PORT_REGISTRY_FILE%"
echo. >> "%PORT_REGISTRY_FILE%"

REM Allocate ports for each service
call :allocate_port "frontend" "frontend"
if defined ALLOCATED_PORT (
    echo FRONTEND_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
    set "FRONTEND_PORT=%ALLOCATED_PORT%"
)

call :allocate_port "backend" "backend"
if defined ALLOCATED_PORT (
    echo BACKEND_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
    set "BACKEND_PORT=%ALLOCATED_PORT%"
)

call :allocate_port "database" "database"
if defined ALLOCATED_PORT (
    echo DB_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
    set "DB_PORT=%ALLOCATED_PORT%"
)

call :allocate_port "redis" "redis"
if defined ALLOCATED_PORT (
    echo REDIS_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
    set "REDIS_PORT=%ALLOCATED_PORT%"
)

call :allocate_port "nginx" "nginx-http"
if defined ALLOCATED_PORT (
    echo HTTP_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
    set "HTTP_PORT=%ALLOCATED_PORT%"
)

call :allocate_port "nginx" "nginx-https"
if defined ALLOCATED_PORT (
    echo HTTPS_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
    set "HTTPS_PORT=%ALLOCATED_PORT%"
)

call :allocate_port "monitoring" "prometheus"
if defined ALLOCATED_PORT (
    echo PROMETHEUS_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
    set "PROMETHEUS_PORT=%ALLOCATED_PORT%"
)

call :allocate_port "monitoring" "grafana"
if defined ALLOCATED_PORT (
    echo GRAFANA_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
    set "GRAFANA_PORT=%ALLOCATED_PORT%"
)

REM Development tools (only for dev environment)
if "%environment%"=="development" (
    call :allocate_port "devtools" "adminer"
    if defined ALLOCATED_PORT (
        echo ADMINER_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
        set "ADMINER_PORT=%ALLOCATED_PORT%"
    )
    
    call :allocate_port "devtools" "redis-commander"
    if defined ALLOCATED_PORT (
        echo REDIS_COMMANDER_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
        set "REDIS_COMMANDER_PORT=%ALLOCATED_PORT%"
    )
    
    call :allocate_port "devtools" "mailhog-web"
    if defined ALLOCATED_PORT (
        echo MAILHOG_WEB_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
        set "MAILHOG_WEB_PORT=%ALLOCATED_PORT%"
    )
    
    call :allocate_port "devtools" "mailhog-smtp"
    if defined ALLOCATED_PORT (
        echo MAILHOG_SMTP_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
        set "MAILHOG_SMTP_PORT=%ALLOCATED_PORT%"
    )
    
    call :allocate_port "devtools" "dev-dashboard"
    if defined ALLOCATED_PORT (
        echo DEV_DASHBOARD_PORT=%ALLOCATED_PORT% >> "%PORT_REGISTRY_FILE%"
        set "DEV_DASHBOARD_PORT=%ALLOCATED_PORT%"
    )
)

REM Add metadata
echo. >> "%PORT_REGISTRY_FILE%"
echo # Port allocation metadata >> "%PORT_REGISTRY_FILE%"
echo PORT_REGISTRY_VERSION=1.0.0 >> "%PORT_REGISTRY_FILE%"
echo PORT_ALLOCATION_DATE=%date% %time% >> "%PORT_REGISTRY_FILE%"
echo PORT_ALLOCATION_ENVIRONMENT=%environment% >> "%PORT_REGISTRY_FILE%"
echo PORT_ALLOCATION_PROJECT=%PROJECT_NAME% >> "%PORT_REGISTRY_FILE%"
echo. >> "%PORT_REGISTRY_FILE%"
echo # Service URLs (for reference) >> "%PORT_REGISTRY_FILE%"

REM Generate service URLs
if defined FRONTEND_PORT echo # Frontend: http://localhost:%FRONTEND_PORT% >> "%PORT_REGISTRY_FILE%"
if defined BACKEND_PORT echo # Backend API: http://localhost:%BACKEND_PORT% >> "%PORT_REGISTRY_FILE%"
if defined GRAFANA_PORT echo # Grafana: http://localhost:%GRAFANA_PORT% >> "%PORT_REGISTRY_FILE%"
if defined PROMETHEUS_PORT echo # Prometheus: http://localhost:%PROMETHEUS_PORT% >> "%PORT_REGISTRY_FILE%"

call :print_success "Port registry generated: %PORT_REGISTRY_FILE%"

REM Display allocated ports
call :print_header "Allocated Ports Summary:"
if defined FRONTEND_PORT echo   FRONTEND_PORT: %FRONTEND_PORT%
if defined BACKEND_PORT echo   BACKEND_PORT: %BACKEND_PORT%
if defined DB_PORT echo   DB_PORT: %DB_PORT%
if defined REDIS_PORT echo   REDIS_PORT: %REDIS_PORT%
if defined HTTP_PORT echo   HTTP_PORT: %HTTP_PORT%
if defined HTTPS_PORT echo   HTTPS_PORT: %HTTPS_PORT%
if defined PROMETHEUS_PORT echo   PROMETHEUS_PORT: %PROMETHEUS_PORT%
if defined GRAFANA_PORT echo   GRAFANA_PORT: %GRAFANA_PORT%
if defined ADMINER_PORT echo   ADMINER_PORT: %ADMINER_PORT%
if defined REDIS_COMMANDER_PORT echo   REDIS_COMMANDER_PORT: %REDIS_COMMANDER_PORT%
if defined MAILHOG_WEB_PORT echo   MAILHOG_WEB_PORT: %MAILHOG_WEB_PORT%
if defined MAILHOG_SMTP_PORT echo   MAILHOG_SMTP_PORT: %MAILHOG_SMTP_PORT%
if defined DEV_DASHBOARD_PORT echo   DEV_DASHBOARD_PORT: %DEV_DASHBOARD_PORT%

goto :eof

REM Function to display current port status
:show_port_status
call :print_header "Current Port Status"

if not exist "%PORT_REGISTRY_FILE%" (
    call :print_warning "No port registry found. Run 'generate' command first."
    goto :eof
)

call :print_status "Port Registry: %PORT_REGISTRY_FILE%"
echo.

REM Parse and display port assignments
for /f "tokens=1,2 delims==" %%a in ('findstr "_PORT=" "%PORT_REGISTRY_FILE%" 2^>nul') do (
    set "key=%%a"
    set "value=%%b"
    
    if defined value (
        set "service_name=!key:_PORT=!"
        call :is_port_available "!value!"
        if "!PORT_AVAILABLE!"=="1" (
            echo   !service_name!: !value! (Available - not in use)
        ) else (
            echo   !service_name!: !value! (In use)
        )
    )
)

echo.
call :print_header "Service URLs:"

REM Display service URLs
for /f "tokens=1,2 delims==" %%a in ('findstr "_PORT=" "%PORT_REGISTRY_FILE%" 2^>nul') do (
    set "key=%%a"
    set "value=%%b"
    
    if "%%a"=="FRONTEND_PORT" echo   🌐 Frontend:        http://localhost:%%b
    if "%%a"=="BACKEND_PORT" echo   🔧 Backend API:     http://localhost:%%b
    if "%%a"=="GRAFANA_PORT" echo   📈 Grafana:         http://localhost:%%b
    if "%%a"=="PROMETHEUS_PORT" echo   📊 Prometheus:      http://localhost:%%b
    if "%%a"=="ADMINER_PORT" echo   🗄️  Database Admin:  http://localhost:%%b
    if "%%a"=="REDIS_COMMANDER_PORT" echo   🔴 Redis Commander: http://localhost:%%b
    if "%%a"=="MAILHOG_WEB_PORT" echo   📧 Mailhog:         http://localhost:%%b
    if "%%a"=="DEV_DASHBOARD_PORT" echo   🛠️  Dev Dashboard:   http://localhost:%%b
)

goto :eof

REM Function to cleanup all port allocations
:cleanup_ports
call :print_status "Cleaning up port allocations..."

REM Remove port registry
if exist "%PORT_REGISTRY_FILE%" (
    del "%PORT_REGISTRY_FILE%"
    call :print_success "Removed port registry"
)

REM Remove all lock files
if exist "%PORT_LOCK_DIR%" (
    rmdir /s /q "%PORT_LOCK_DIR%"
    call :print_success "Removed port locks"
)

call :print_success "Port cleanup completed"
goto :eof

REM Function to regenerate ports
:regenerate_ports
set "environment=%~1"
if not defined environment set "environment=development"

call :print_status "Regenerating port allocations for %environment% environment..."

REM Cleanup existing allocations
call :cleanup_ports

REM Generate new allocations
call :generate_port_registry "%environment%"

call :print_success "Port regeneration completed"
goto :eof

REM Function to show usage
:show_usage
echo Free Deep Research System - Port Manager (Windows)
echo.
echo Usage: %~nx0 ^<command^> [options]
echo.
echo Commands:
echo     generate ^<env^>     Generate port registry for environment (development^|production)
echo     status             Show current port status and service URLs
echo     cleanup            Clean up all port allocations
echo     regenerate ^<env^>   Regenerate port allocations for environment
echo     help               Show this help message
echo.
echo Examples:
echo     %~nx0 generate development
echo     %~nx0 status
echo     %~nx0 cleanup
echo     %~nx0 regenerate production
echo.
goto :eof

REM Main execution
:main
set "command=%~1"
if not defined command set "command=help"

if "%command%"=="generate" (
    call :generate_port_registry "%~2"
    goto :eof
)

if "%command%"=="status" (
    call :show_port_status
    goto :eof
)

if "%command%"=="cleanup" (
    call :cleanup_ports
    goto :eof
)

if "%command%"=="regenerate" (
    call :regenerate_ports "%~2"
    goto :eof
)

if "%command%"=="help" (
    call :show_usage
    goto :eof
)

call :print_error "Unknown command: %command%"
call :show_usage
goto :eof

REM Execute main function
call :main %*
