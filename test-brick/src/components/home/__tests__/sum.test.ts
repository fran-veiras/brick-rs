import { expect } from '@jest/globals';
import { sum } from '../utils/sum/sum';

test('two plus two is four', () => {
	expect(sum(2, 2)).toBe(4);
});
