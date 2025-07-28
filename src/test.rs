#![allow(dead_code)]

static BASIC_INPUT: &'static str = "
Project 'ApplicationLayerClient' has the following package references
   [net9.0]: 
   Top-level Package                               Requested   Resolved
   > Autofac                                       8.2.1       8.2.1   
   > Microsoft.Extensions.DependencyInjection      9.0.7       9.0.7   

   Transitive Package                                           Resolved
   > Microsoft.Extensions.DependencyInjection.Abstractions      9.0.7   
   > System.Diagnostics.DiagnosticSource                        8.0.1   

Project 'ApplicationLayer' has the following package references
   [net9.0]: 
   Top-level Package                               Requested   Resolved
   > Microsoft.Extensions.DependencyInjection      9.0.7       9.0.7   

   Transitive Package                                           Resolved
   > Microsoft.Extensions.DependencyInjection.Abstractions      9.0.7   
";

static SERILOG_INPUT: &'static str = "
Project 'Package' has the following package references
   [net8.0-windows7.0]:
   Top-level Package      Requested   Resolved
   > OneOf                3.0.271     3.0.271

   Transitive Package           Resolved
   > MathNet.Numerics           5.0.0
   > Serilog                    4.2.0
   > Serilog.Sinks.Console      6.0.0
   > Serilog.Sinks.Debug        3.0.0
   > Serilog.Sinks.File         6.0.0
";

#[cfg(test)]
mod tests {
    use std::collections::*;
    use crate::parser::*;
    use super::*; 

    #[test]
    fn test_first_four_packages() {
        let result = parse_packages(BASIC_INPUT);

        assert_eq!(result[0], "Autofac"); 
        assert_eq!(result[1], "Microsoft.Extensions.DependencyInjection"); 
        assert_eq!(result[2], "Microsoft.Extensions.DependencyInjection.Abstractions"); 
        assert_eq!(result[3], "System.Diagnostics.DiagnosticSource"); 
    }

    #[test]
    fn test_only_four_packages() {
        let result = parse_packages(BASIC_INPUT);
        
        assert_eq!(result.len(), 4); 
    }

    #[test]
    fn test_empty() {
        let result = parse_packages("");
        assert_eq!(result.len(), 0); 
        
        let result = parse_packages("    ");
        assert_eq!(result.len(), 0); 
    }

    #[test]
    fn test_package_table() {
        let packages = parse_packages(BASIC_INPUT);
        let result = create_package_table(packages);

        assert_eq!(result.len(), 4); 

        assert_eq!(result[0].len(), 1); 
        assert_eq!(result[0][0], "Autofac"); 

        assert_eq!(result[1].len(), 3); 
        assert_eq!(result[1][0], "Microsoft"); 
        assert_eq!(result[1][1], "Extensions"); 
        assert_eq!(result[1][2], "DependencyInjection"); 

        assert_eq!(result[2].len(), 4); 
        assert_eq!(result[2][0], "Microsoft"); 
        assert_eq!(result[2][1], "Extensions"); 
        assert_eq!(result[2][2], "DependencyInjection"); 
        assert_eq!(result[2][3], "Abstractions"); 

        assert_eq!(result[3].len(), 3); 
        assert_eq!(result[3][0], "System"); 
        assert_eq!(result[3][1], "Diagnostics"); 
        assert_eq!(result[3][2], "DiagnosticSource"); 
    }

    #[test]
    fn test_package_map() {
        let packages = parse_packages(BASIC_INPUT);
        let map = create_package_map(packages);

        assert_eq!(map.len(), 3); 

        assert_eq!(map["Autofac"][0], "Autofac");

        assert_eq!(map["Microsoft"][0], "Microsoft.Extensions.DependencyInjection");
        assert_eq!(map["Microsoft"][1], "Microsoft.Extensions.DependencyInjection.Abstractions");

        assert_eq!(map["System"][0], "System.Diagnostics.DiagnosticSource");
    }

    #[test]
    fn test_masked_packages() {
        let packages = parse_packages(BASIC_INPUT);
        let map = create_package_map(packages);
        let masked_packages = create_masked_packages(map);
        
        assert_eq!(masked_packages.len(), 3);
        assert_eq!(masked_packages[0], "Autofac"); 
        assert_eq!(masked_packages[1], "Microsoft.*"); 
        assert_eq!(masked_packages[2], "System.*"); 
    }

    #[test]
    fn test_broken_masked_packages() {
        let mut map: HashMap<&str, Vec<&str>>  = HashMap::new();
        map.insert("hello", vec![]);

        let masked_packages = create_masked_packages(map);

        assert_eq!(masked_packages.len(), 0);
    }

    #[test]
    fn test_separated_masked_packages() {
        let packages = parse_packages(SERILOG_INPUT);
        let map = create_package_map(packages);
        let masked_packages = create_masked_packages(map);

        assert_eq!(masked_packages.len(), 4);
        assert_eq!(masked_packages[0], "MathNet.*"); 
        assert_eq!(masked_packages[1], "OneOf"); 
        assert_eq!(masked_packages[2], "Serilog"); 
        assert_eq!(masked_packages[3], "Serilog.*"); 
    }

    #[test]
    fn test_packages_patterns() {
        let packages = parse_packages(SERILOG_INPUT);
        let map = create_package_map(packages);
        let masked_packages = create_masked_packages(map);
        let patterns = create_package_patterns(masked_packages);

        assert_eq!(patterns.len(), 4);
        assert_eq!(patterns[0], "<package pattern=\"MathNet.*\" />"); 
        assert_eq!(patterns[1], "<package pattern=\"OneOf\" />"); 
        assert_eq!(patterns[2], "<package pattern=\"Serilog\" />"); 
        assert_eq!(patterns[3], "<package pattern=\"Serilog.*\" />"); 
    }
}