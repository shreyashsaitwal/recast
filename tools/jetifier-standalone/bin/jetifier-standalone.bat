@rem Copyright 2015 the original author or authors.
@rem
@rem Licensed under the Apache License, Version 2.0 (the "License");
@rem you may not use this file except in compliance with the License.
@rem You may obtain a copy of the License at
@rem
@rem      https://www.apache.org/licenses/LICENSE-2.0
@rem
@rem Unless required by applicable law or agreed to in writing, software
@rem distributed under the License is distributed on an "AS IS" BASIS,
@rem WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
@rem See the License for the specific language governing permissions and
@rem limitations under the License.
@if "%DEBUG%" == "" @echo off
if "%OS%"=="Windows_NT" setlocal
set DIRNAME=%~dp0
if "%DIRNAME%" == "" set DIRNAME=.
set APP_BASE_NAME=%~n0
set APP_HOME=%DIRNAME%..
set DEFAULT_JVM_OPTS=
if defined JAVA_HOME goto findJavaFromJavaHome
set JAVA_EXE=java.exe
%JAVA_EXE% -version >NUL 2>&1
if "%ERRORLEVEL%" == "0" goto init
echo.
echo ERROR: JAVA_HOME is not set and no 'java' command could be found in your PATH.
echo.
echo Please set the JAVA_HOME variable in your environment to match the
echo location of your Java installation.
goto fail
:findJavaFromJavaHome
set JAVA_HOME=%JAVA_HOME:"=%
set JAVA_EXE=%JAVA_HOME%/bin/java.exe
if exist "%JAVA_EXE%" goto init
echo.
echo ERROR: JAVA_HOME is set to an invalid directory: %JAVA_HOME%
echo.
echo Please set the JAVA_HOME variable in your environment to match the
echo location of your Java installation.
goto fail
:init
if not "%OS%" == "Windows_NT" goto win9xME_args
:win9xME_args
set CMD_LINE_ARGS=
set _SKIP=2
:win9xME_args_slurp
if "x%~1" == "x" goto execute
set CMD_LINE_ARGS=%*
:execute
set CLASSPATH=%APP_HOME%\lib\jetifier-standalone.jar;%APP_HOME%\lib\jetifier-processor-1.0.0-beta09.jar;%APP_HOME%\lib\commons-cli-1.3.1.jar;%APP_HOME%\lib\jetifier-core-1.0.0-beta09.jar;%APP_HOME%\lib\asm-util-6.0.jar;%APP_HOME%\lib\asm-commons-6.0.jar;%APP_HOME%\lib\asm-tree-6.0.jar;%APP_HOME%\lib\asm-6.0.jar;%APP_HOME%\lib\jdom2-2.0.6.jar;%APP_HOME%\lib\kotlin-stdlib-1.3.60.jar;%APP_HOME%\lib\gson-2.8.0.jar;%APP_HOME%\lib\kotlin-stdlib-common-1.3.60.jar;%APP_HOME%\lib\annotations-13.0.jar
"%JAVA_EXE%" %DEFAULT_JVM_OPTS% %JAVA_OPTS% %JETIFIER_STANDALONE_OPTS%  -classpath "%CLASSPATH%" com.android.tools.build.jetifier.standalone.Main %CMD_LINE_ARGS%
:end
if "%ERRORLEVEL%"=="0" goto mainEnd
:fail
if  not "" == "%JETIFIER_STANDALONE_EXIT_CONSOLE%" exit 1
exit /b 1
:mainEnd
if "%OS%"=="Windows_NT" endlocal
:omega
