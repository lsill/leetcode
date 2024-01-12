file(GLOB LC_TEST_SOURCES "${PROJECT_SOURCE_DIR}/src/test/*.cpp")


foreach (test_source ${LC_TEST_SOURCES})
    # Create a human readable name.
    get_filename_component(test_filename ${test_source} NAME)
    string(REPLACE ".cpp" "" test_name ${test_filename})

    add_executable(${test_name} EXCLUDE_FROM_ALL ${test_source})
    #add_dependencies(build-tests ${test_filename})
    #add_dependencies(check-tests ${test_filename})

    set(libpath "${CMAKE_SOURCE_DIR}/build/lib/lib")
    string(APPEND libpath ${test_name})
    string(APPEND libpath "_pra.a")

    target_link_libraries(${test_name} ${libpath}) #${CMAKE_SOURCE_DIR}/build/lib/libstack_pra.a
    # Set test target properties and dependencies.
    set_target_properties(${test_name}
            PROPERTIES
            RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
            COMMAND ${test_name}
            )

endforeach (test_source ${x})