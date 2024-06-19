use std::collections::HashMap;

// Yeah this is the best solution I could come up with for now lol

pub struct Languages {
    pub id_map: HashMap<String, u8>
}

impl Default for Languages {
    fn default() -> Self {
        let mut mp = HashMap::new();
        mp.insert("GNU GCC C11 5.1.0".to_string(), 43);
        mp.insert("GNU G++14 6.4.0".to_string(), 50);
        mp.insert("GNU G++17 7.3.0".to_string(), 54);
        mp.insert("GNU G++20 13.2 (64 bit, winlibs)".to_string(), 89);
        mp.insert("C# 8, .NET Core 3.1".to_string(), 65);
        mp.insert("C# 10, .NET SDK 6.0".to_string(), 79);
        mp.insert("C# Mono 6.8".to_string(), 9);
        mp.insert("D DMD32 v2.105.0".to_string(), 28);
        mp.insert("Go 1.22.2".to_string(), 32);
        mp.insert("Haskell GHC 8.10.1".to_string(), 12);
        mp.insert("Java 21 64bit".to_string(), 87);
        mp.insert("Java 8 32bit".to_string(), 36);
        mp.insert("Kotlin 1.7.20".to_string(), 83);
        mp.insert("Kotlin 1.9.21".to_string(), 88);
        mp.insert("OCaml 4.02.1".to_string(), 19);
        mp.insert("Delphi 7".to_string(), 3);
        mp.insert("Free Pascal 3.2.2".to_string(), 4);
        mp.insert("PascalABC.NET 3.8.3".to_string(), 51);
        mp.insert("Perl 5.20.1".to_string(), 13);
        mp.insert("PHP 8.1.7".to_string(), 6);
        mp.insert("Python 2.7.18".to_string(), 7);
        mp.insert("Python 3.8.10".to_string(), 31);
        mp.insert("PyPy 2.7.13 (7.3.0)".to_string(), 40);
        mp.insert("PyPy 3.6.9 (7.3.0)".to_string(), 41);
        mp.insert("PyPy 3.10 (7.3.15, 64bit)".to_string(), 70);
        mp.insert("Ruby 3.2.2".to_string(), 67);
        Languages {
            id_map: mp
        }
    }
}
