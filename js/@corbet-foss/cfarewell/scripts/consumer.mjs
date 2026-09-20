// Copy next to a fresh installation to test the actual npm tarball.
import { createRequire } from 'node:module';
import { verify } from './verify-api.mjs';
verify(await import('@corbet-foss/cfarewell'));
verify(createRequire(import.meta.url)('@corbet-foss/cfarewell'));
verify(await import('@corbet-foss/cfarewell/browser'));
console.log('cfarewell: installed ESM, CommonJS, and browser exports passed');
