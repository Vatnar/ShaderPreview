cmake_minimum_required(VERSION 3.30)
project(ShaderViewer)

set(CMAKE_CXX_STANDARD 20)

# Set output directory for binaries
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Path to the ImGui source folder
set(IMGUI_DIR "vendor/imgui")

# Include the ImGui directory and backends
include_directories(${IMGUI_DIR})
include_directories(${IMGUI_DIR}/backends)

# Find glfw3, make sure you have it installed via vcpkg or another package manager
find_package(glfw3 CONFIG REQUIRED)

# List of backend source files
set(IMGUI_BACKENDS
		${IMGUI_DIR}/backends/imgui_impl_dx11.cpp
		${IMGUI_DIR}/backends/imgui_impl_glfw.cpp
)

# Create the executable from your main.cpp and backend files
add_executable(${CMAKE_PROJECT_NAME} src/main.cpp ${IMGUI_BACKENDS})

# Link the necessary libraries: GLFW, Direct3D, and the necessary DX libraries
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE
		glfw3
		d3d11.lib
		dxgi.lib
		dxguid.lib
)