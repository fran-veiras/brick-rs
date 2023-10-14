import { expect } from '@jest/globals';
import { division } from './division';

test('two plus two is four', () => {
	expect(division(4, 2)).toBe(2);
});
