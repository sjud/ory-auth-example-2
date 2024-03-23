@logout
Feature: Logout

    As a user
    I want to log out after registering
    So that I can test to see if login after registering works.

    
    Scenario: Register, verify and log in a user and then log out.
        Given I am on the registration page
        And I see the registration form
        And I enter valid credentials
        And I check my email for the verification link and code
        And I copy the code onto the verification link page
        And I click login
<<<<<<< HEAD
        And I re-enter valid credentials
=======
        And I enter valid credentials
        And I am on the homepage
>>>>>>> e1b880d (idk)
        When I click logout
        Then I am logged out