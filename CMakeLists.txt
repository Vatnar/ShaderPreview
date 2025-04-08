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

set(GLFW_DIR "vendor/glfw")
include_directories(${GLFW_DIR}/include)
set(IMGUI_BACKENDS
		${IMGUI_DIR}/backends/imgui_impl_dx11.cpp
		${IMGUI_DIR}/backends/imgui_impl_glfw.cpp
)

# Add GLFW source files if you're building it from source
add_subdirectory(${GLFW_DIR})
# Create the executable from your main.cpp and backend files
add_executable(${CMAKE_PROJECT_NAME} src/main.cpp ${IMGUI_BACKENDS} ${GLFW_DIR}/src)

# Link the necessary libraries: GLFW, Direct3D, and the necessary DX libraries
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE
		glfw
		d3d11.lib
		dxgi.lib
		dxguid.lib
)
