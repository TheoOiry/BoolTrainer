// https://docs.cypress.io/api/introduction/api.html

describe('My First Test', () => {
  it('User can start a new game', () => {
    cy.visit('/')
    cy.contains('h1', 'Bool trainer')
    cy.contains('p', 'The first training application for boolean expressions.')
    cy.get('#buttonStart').click()
    cy.url().should('include', '/game')
  })

  it('should start', () => {
    cy.visit('/')
    cy.get('#buttonStart').click()
    cy.get('.boxShadowCode', )
  });
})
