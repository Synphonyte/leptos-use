import { test, expect } from '@playwright/test';

test.describe('BroadcastChannelDemo SSR', () => {
  test('shows fallback after SSR and hydration', async ({ page }) => {
    await page.goto('http://localhost:3000/broadcast-channel', { waitUntil: 'networkidle' });

    // Reload to simulate SSR hydration
    await page.reload({ waitUntil: 'networkidle' });

    // Check that the fallback is visible
    await expect(page.locator('text=BroadcastChannel not supported')).toBeVisible();

    // Optionally: Check for hydration errors in the browser console
    const logs: string[] = [];
    page.on('console', msg => logs.push(msg.text()));
    await page.reload({ waitUntil: 'networkidle' });
    expect(logs.filter(l => l.includes('hydration error'))).toHaveLength(0);
  });
});