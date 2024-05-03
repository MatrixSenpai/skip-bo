import { createRoot } from 'react-dom/client';
import * as _ from 'bootstrap';

import 'assets/css/styles.scss';

import { App } from 'app';

const documentRoot = document.getElementById('root')!;
const root = createRoot(documentRoot);

root.render(<App />);
