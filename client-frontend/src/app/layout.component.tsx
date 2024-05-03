import {FunctionComponent} from "react";
import {Outlet} from "react-router-dom";

interface Props {
}

export const Layout: FunctionComponent<Props> = ({ }) => {
    return (
        <Outlet />
    )
}