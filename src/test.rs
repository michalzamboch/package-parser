
#[allow(dead_code)]

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
    use crate::parser::parse_packages;

    use super::*; 

    #[test]
    fn test_add_positive_numbers() {
        let result = parse_packages(BASIC_INPUT);

        assert_eq!(result[0], "Autofac"); 
        assert_eq!(result[1], "Microsoft.Extensions.DependencyInjection"); 
        assert_eq!(result[2], "Microsoft.Extensions.DependencyInjection.Abstractions"); 
        assert_eq!(result[3], "System.Diagnostics.DiagnosticSource"); 
    }
}