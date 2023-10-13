describe('template spec', () => {
	before(() => {
		cy.visit('/');
	});

	it('passes', () => {
		cy.get('#to-test').click({ force: true });
		cy.get('#to-home').click({ force: true });
		cy.get('#to-test').click({ force: true });
	});
});
