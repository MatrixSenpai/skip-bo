import { FunctionComponent, StrictMode } from "react";
import { RouterProvider } from "react-router-dom";

import { memoryRouter } from './router';

export const App: FunctionComponent = () => {
    return (
        <StrictMode>
            <RouterProvider router={memoryRouter} />
        </StrictMode>
    )
}