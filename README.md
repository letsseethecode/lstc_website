# Outline

## Technologies

* Rust (WebASM & RESTful API)
* Terraform
* AWS
* Html
* Javascript

# Instructions

## How to install


Install the pre-requisites using.
```sh
make install
```
Now we must run terraform to generate the assets, such as the Elastic Container
Registries and DNS records.  We will need the ECRs specifically in the
subsequent steps.  The `DOMAIN` parameter is optional and can be used to
specify any domain name you like.
```sh
DOMAIN=letsseethecode.com SCOPE=per_account make plan
SCOPE=per_account make apply
```

Build the project
```sh
make build
```

Generate the Terraform plan
```sh
make plan
```

Apply the Terraform plan
```sh
make apply
```

# Architecture

## C4 Diagram

### C1 - Context

![C4 Diagram - Context](./docs/images/C4-C1%20-%20Context.png)

### C2 - Containers

![C4 Diagram - Website](./docs/images/C4-C2%20-%20website.png)

## Infrastructure

![AWS Infrastructure](./docs/images/C4-AWS%20Components.drawio.png)

# Features

1. **As a** Visitor **I want to** register using github.com, so that I can authenticate

    Given "Code is the Star" it mames sense that we federate our authentication to use github.com.

    ---

2. **As a** Visitor **I want to** login, so that I can access Member/Administrator features

    As an unauthenticated user I want to login to the website so that I can access Member/Administrator features.

    ---

3. **As a** Visitor **I want to** browse upcoming events

    Anyone visiting the site must be able to see events that are scheduled.

    1. **As a** Visitor **I want to** see a summary of upcoming and past events

        We're going to need a list of all the available events.
   
        ---
   
    2. **As a** Vistor **I want to** see a detailed view of an event
    
        Once we've seen an event we want to be able to click it and see detail.

        ---

4. **As a** Member **I want to** attend events

    Once we've seen an event we're interested we need to be able to attend it.

    1. **As a** Member **I want to** register my attendance

        First step is to register intent to attend the meeting.

        ---

    2. **As a** Member **I want to** cancel my attendance

        Whoops.  We need to cancel it.

        ---

    3. **As a** Member **I want to** give feedback on an event I attended

        We want to generate some noise and interest in the event.

        1. **As a** Member **I want to** hype up the event

            Thumbs Up/Emoticons/Vibes/etc

            ---

        2. **As a** Member **I want to** upload photos from the event

            It'd be great to be able to see pictures of the event that people have uploaded.

            ---

5. **As an** Administrator **I want to** manage Events

    1. **As an** Administrator **I want to** register a new event
   
        A key feature of the site is registering new events.
        
        ---

    2. **As an** Administrator **I want to** close an exising event
   
        Events that are opened may need to be deleted.
        
        ---

    3. **As an** Administrator **I want to** edit an exising event

        Change the details of an event.
        
        ---

    4. **As an** Administrator **I want to** move an existing event

        Moving an event will cause it to be reindex, so it's the same as another "edit" function.
        
        ---
