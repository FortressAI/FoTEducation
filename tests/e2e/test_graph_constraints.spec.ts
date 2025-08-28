import { test, expect } from '@playwright/test';

test.describe('FoT Knowledge Graph Constraints', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the system
    await page.goto('http://localhost:8080');
  });

  test('should enforce SHACL constraints on claim creation', async ({ page }) => {
    // Test that superposed claims must have provenance
    const response = await page.request.post('/api/claims', {
      data: {
        content: 'Test claim',
        state: 'superposed',
        // Missing provenance - should fail
      }
    });

    expect(response.status()).toBe(400);
    expect(await response.text()).toContain('provenance');
  });

  test('should enforce virtue thresholds for claim verification', async ({ page }) => {
    // Test that claim verification requires high honesty
    const response = await page.request.post('/api/claims/verify', {
      data: {
        claim_id: 'test_claim',
        verifier_id: 'low_honesty_user',
        // User with low honesty - should fail
      }
    });

    expect(response.status()).toBe(403);
    expect(await response.text()).toContain('honesty');
  });

  test('should enforce role-based access control', async ({ page }) => {
    // Test that parents cannot create claims
    const response = await page.request.post('/api/claims', {
      data: {
        content: 'Test claim',
        state: 'superposed',
        creator_id: 'parent_user',
        // Parent trying to create claim - should fail
      }
    });

    expect(response.status()).toBe(403);
    expect(await response.text()).toContain('read-only');
  });
});
