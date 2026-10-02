

# prerequisites for azure admins
* IaS: json, biceps, ARM templates

## ARM templates
* idem potent
* declarative
* tries to create resources in parallel

element,desc
schema,
contentVersion
apiProfile
prameters
variables
functions
resources
output

sku -> stock-keeping unit 



## azure CLI commands
* az account list-locations
* az configure --defaults <group={group}> <location={location}>
* az group create --name <name> --location "<loc>"
* az deployment group create


# manage identities and governance in azure
* microsoft entra ID part of PaaS directory service
* identity solution, 80/443
* multitenant directory service
* users / groups -> flat structure
