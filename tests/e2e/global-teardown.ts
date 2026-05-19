import { execSync } from 'child_process';
import * as path from 'path';

export default async function globalTeardown() {
  execSync(`bash ${path.join(__dirname, '../common/stop-db.sh')}`, { stdio: 'inherit' });
}
