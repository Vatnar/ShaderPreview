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

# ImGui source files (including the backends)
set(IMGUI_SRC
		${IMGUI_DIR}/imgui.cpp
		${IMGUI_DIR}/imgui_demo.cpp
		${IMGUI_DIR}/imgui_draw.cpp
		${IMGUI_DIR}/imgui_tables.cpp
		${IMGUI_DIR}/imgui_widgets.cpp
)

# ImGui backend source files (including DX11 and Win32)
set(IMGUI_BACKEND_SRC
		${IMGUI_DIR}/backends/imgui_impl_dx11.cpp
		${IMGUI_DIR}/backends/imgui_impl_glfw.cpp
		${IMGUI_DIR}/backends/imgui_impl_win32.cpp
)

# Add GLFW source files if you're building it from source
add_subdirectory(${GLFW_DIR})

# Create the executable from your main.cpp and backend files
add_executable(${CMAKE_PROJECT_NAME} src/main.cpp ${IMGUI_SRC} ${IMGUI_BACKEND_SRC})

# Link the necessary libraries: GLFW, Direct3D, and the necessary DX libraries
target_link_libraries(ShaderViewer
		PRIVATE
		glfw
		d3d11
		dxgi
		dxguid
		d3dcompiler
		dwmapi      # Add this line
		kernel32
		user32
		gdi32
		winspool
		shell32
		ole32
		oleaut32
		uuid
		comdlg32
		advapi32
)
