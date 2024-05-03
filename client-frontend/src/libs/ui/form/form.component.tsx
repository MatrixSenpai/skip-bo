import { FunctionComponent, ReactElement } from 'react';
import { Formik, Form as FormikForm } from 'formik';

interface Props {
    initialValues: Record<string, any>;
    children: ReactElement | ReactElement[];
    handleFormSubmission: (values: Record<string, any>) => void;
    validate?: (values: Record<string, any>) => Record<string, any>;
}

export const Form: FunctionComponent<Props> = ({ children, initialValues, handleFormSubmission, validate }) => {
    return (
        <Formik initialValues={initialValues} onSubmit={handleFormSubmission} validateOnBlur={true} validate={validate}>
            <FormikForm>
                {children}
            </FormikForm>
        </Formik>
    )
}