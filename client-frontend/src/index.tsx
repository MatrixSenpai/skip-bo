import { createRoot } from 'react-dom/client';
import * as _ from 'bootstrap';

import 'assets/css/styles.scss';

const documentRoot = document.getElementById('root')!;
const root = createRoot(documentRoot);

root.render(<h1>Hello</h1>);
