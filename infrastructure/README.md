# Infrastructure

This part of the project contains all the code required to deploy the site.

Terminology:

* Account : A single AWS account that hosts the solution
* Environment: A single instance of the software (e.g. DEV, TEST, UAT, PROD, etc.)
* Scope: A set of Infrastructure As Code (IaC) that is required, and is separate from the whole.

There are three scopes of IaC in the project.

1. bootstrap the project from nothing.  This is written in bash scripts, as it's main responsibility is creating the terraform assets.
2. apply any account-wide settings that may be shared across environments.  e.g. Some cloudwatch settings are account-wide, and shared across each environment.
3. apply the environment specific settings that are required.  This is where the bulk of the application is going to reside.

