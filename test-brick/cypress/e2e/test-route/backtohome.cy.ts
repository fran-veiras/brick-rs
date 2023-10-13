describe('template spec', () => {
	before(() => {
		cy.visit('/');
	});

	it('passes', () => {
		cy.get('#to-home').click({ force: true });
	});
});
