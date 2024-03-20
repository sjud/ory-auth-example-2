@login
Feature: Login

    As a user
    I want to log in
    So that I can get access to authorized content.

    Background:
        Given I am on the homepage
    
    @test-login-green-path
    Scenario: Register, verify and log in a user.
        Given I am on the registration page
        And I see the registration form
        And I enter valid credentials
        And I check my email for the verification link and code
        And I copy the code onto the verification link page
        When I click login
        And I enter valid credentials
        Then I am on the homepage
        And I see logout button