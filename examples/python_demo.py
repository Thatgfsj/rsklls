"""
Python Demo - Calling Rust from Python (PyO3)

This example demonstrates how to use the Rust library from Python.
"""

from my_rust_lib import greet, add, greet_chinese, Person


def main():
    # Function calls
    print("=== Function Examples ===")
    result = greet("World")
    print(f"greet('World'): {result}")
    
    sum_result = add(1, 2)
    print(f"add(1, 2): {sum_result}")
    
    # Chinese support
    chinese_greeting = greet_chinese("张三")
    print(f"greet_chinese('张三'): {chinese_greeting}")
    
    print("\n=== Class Examples ===")
    
    # Create a Person object
    person = Person("李四", 25)
    
    # Call methods
    print(f"person.greet(): {person.greet()}")
    print(f"person.greet_chinese(): {person.greet_chinese()}")
    print(f"person.get_age(): {person.get_age()}")
    
    # Modify age
    person.set_age(26)
    print(f"After set_age(26), get_age(): {person.get_age()}")
    
    print("\n=== Demo Complete ===")


if __name__ == "__main__":
    main()
