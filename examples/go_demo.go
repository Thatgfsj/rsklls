// Go Demo - Calling Rust from Go (cgo)
package main

/*
#cgo LDFLAGS: -L./target/release -lmy_rust_lib
#include <stdlib.h>
#include <stdint.h>

// Forward declarations
int32_t rust_add(int32_t a, int32_t b);
char* rust_greet(char* name);
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	fmt.Println("=== Rust-Go Interop Demo ===")

	// Call Rust add function
	a := C.int(1)
	b := C.int(2)
	result := C.rust_add(a, b)
	fmt.Printf("rust_add(%d, %d) = %d\n", a, b, result)

	// Call Rust greet function
	name := C.CString("World")
	defer C.free(unsafe.Pointer(name))
	greeting := C.rust_greet(name)
	defer C.free(unsafe.Pointer(greeting))
	fmt.Printf("rust_greet(\"World\") = %s\n", C.GoString(greeting))

	// Chinese support test
	nameCN := C.CString("张三")
	defer C.free(unsafe.Pointer(nameCN))
	greetingCN := C.rust_greet(nameCN)
	defer C.free(unsafe.Pointer(greetingCN))
	fmt.Printf("rust_greet(\"张三\") = %s\n", C.GoString(greetingCN))

	fmt.Println("\n=== Demo Complete ===")
}
