# TODO

## General

- [ ] Implement unit tests for each module.
  - [ ] Error Handling
  - [ ] Modules
  - [ ] Parser
  - [ ] Scraper
  - [ ] Excel
- [ ] Set up continuous integration for automated testing?
- [ ] Document each struct and its fields.
- [ ] Write comprehensive documentation for each module.
- [ ] Document public functions and structs.
- [ ] Create a README file with project setup and usage instructions.

## Scraper Module (scraper)

- [ ] Implement the HTTP request function to fetch HTML content.
  - [ ] Check all a-tags, one layer for content
    - [ ] Get content from forms/POST-requests
- [ ] Add error handling

## Parser Module (parser)

- [ ] Parse URL
  - [ ] Get medium
  - [ ] Get id
- [ ] Develop functions to parse HTML content
  - [x] Get title
  - [x] Get text
  - [x] Get forms
  - [x] Get all links
    - [x] Seperate into external and internal
    - [x] Seperate internal into sok and non-sok
    - [x] Seperate into partial and full URLs
  - [x] Get all tables
  - [ ] Get metoder
  - [ ] Get kilder
  - [ ] Get merknad
- [ ] Handle parsing errors and unexpected HTML structures.
  - [ ] Add logging?

## Excel Conversion Module (xl)

- [ ] Create functions to format data into Excel sheets.
- [ ] Implement different data types and formatting requirements handling.
- [ ] Add error handling for Excel file writing and formatting issues.

## Error Module (error.rs)

- [ ] Define custom error types for different modules.
- [ ] Implement `Display` and `Error` traits for custom errors.
- [ ] Ensure clear and descriptive error messages.
- [ ] Implement From for each used error.
