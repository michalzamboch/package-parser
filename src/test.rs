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

#[cfg(test)]
mod tests {
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
}