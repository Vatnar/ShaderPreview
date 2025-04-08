cmake_minimum_required(VERSION 3.30)
project(ShaderViewer)
set(CMAKE_CXX_STANDARD 20)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

find_package(glfw3 CONFIG REQUIRED)
find_package(imgui CONFIG REQUIRED)
add_executable(${CMAKE_PROJECT_NAME} src/main.cpp)

target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE
	glfw
	imgui::imgui
	d3d11
	dxgi
	dxguid
)