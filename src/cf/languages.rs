use std::collections::HashMap;

// Yeah this is the best solution I could come up with for now lol

pub struct Languages {
    pub id_map: HashMap<String, u8>
}

impl Default for Languages {
    fn default() -> Self {
        Languages {
            id_map: HashMap::from([
                (String::from("GNU GCC C11 5.1.0"), 43),
                (String::from("GNU G++14 6.4.0"), 50),
                (String::from("GNU G++17 7.3.0"), 54),
                (String::from("GNU G++20 13.2 (64 bit, winlibs)"), 89),
                (String::from("C# 8, .NET Core 3.1"), 65),
                (String::from("C# 10, .NET SDK 6.0"), 79),
                (String::from("C# Mono 6.8"), 9),
                (String::from("D DMD32 v2.105.0"), 28),
                (String::from("Go 1.22.2"), 32),
                (String::from("Haskell GHC 8.10.1"), 12),
                (String::from("Java 21 64bit"), 87),
                (String::from("Java 8 32bit"), 36),
                (String::from("Kotlin 1.7.20"), 83),
                (String::from("Kotlin 1.9.21"), 88),
                (String::from("OCaml 4.02.1"), 19),
                (String::from("Delphi 7"), 3),
                (String::from("Free Pascal 3.2.2"), 4),
                (String::from("PascalABC.NET 3.8.3"), 51),
                (String::from("Perl 5.20.1"), 13),
                (String::from("PHP 8.1.7"), 6),
                (String::from("Python 2.7.18"), 7),
                (String::from("Python 3.8.10"), 31),
                (String::from("PyPy 2.7.13 (7.3.0)"), 40),
                (String::from("PyPy 3.6.9 (7.3.0)"), 41),
                (String::from("PyPy 3.10 (7.3.15, 64bit)"), 70),
                (String::from("Ruby 3.2.2"), 67),
            ]),
        }
    }
}
