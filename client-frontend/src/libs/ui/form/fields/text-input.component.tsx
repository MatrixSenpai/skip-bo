import { FunctionComponent, ReactElement } from 'react';
import { Field, useField } from 'formik';

interface Props {
    name: string;
    type: string;
    label: string | ReactElement;
    helpText?: string | ReactElement;
    disabled?: boolean;
}

export const TextInput: FunctionComponent<Props> = ({
    name,
    type,
    label,
    helpText,
    disabled,
}) => {
    const [_field, meta] = useField(name);

    return (
        <div className='mb-3'>
            <label className='form-label'>{label}</label>
            <Field
                className={`form-control ${meta.touched && meta.error && 'is-invalid'}`}
                type={type}
                name={name}
                disabled={disabled}
            />
            {helpText && <div className='form-text'>{helpText}</div>}
            {meta.touched && meta.error && (
                <div className='invalid-feedback'>{meta.error}</div>
            )}
        </div>
    );
};
