import type { Ref, ComputedRef } from "vue";
import { computed, onMounted, ref } from "vue";

export type RuleName = string;
export type RuleResponse = [boolean, string];
export type RuleFunction = [RuleName, (input: string | number) => RuleResponse];
export type FormErrorMessage = string;
export type FormErrors = Record<RuleName, FormErrorMessage>;

export interface FormValidation {
  /**
   * Errors grouped by field name.
   *
   * Example:
   * { name: { "required": "Field is required" } }
   */
  errors: Ref<Record<string, FormErrors>>;

  /**
   * Current status of the form, if valid or not
   */
  isValidForm: ComputedRef<boolean>;

  /**
   * List of available rules
   */
  rules: Record<string, (msg?: string) => RuleFunction>;

  /**
   * Function to run all validations
   *
   * @param request form input (key, value)
   * @param validations list of validations (field, list of rules)
   */
  validate(
    request: Record<string, any>,
    validations: Record<string, RuleFunction[]>
  ): void;
}

// Rule to check if a required field is empty or not
function required(msg: string = "Field is required"): RuleFunction {
  return [
    "required",
    (input: string | number): RuleResponse => {
      return [input?.toString().trim() !== "", msg];
    },
  ];
}

export function useFormValidation(): FormValidation {
  const errors = ref<Record<string, Record<string, string>>>({});

  const isValidForm = computed(() => Object.keys(errors.value).length === 0);

  const validateField = (value: any, field: string, rules: RuleFunction[]) => {
    for (const [ruleName, ruleFn] of rules) {
      const [isValid, errorMessage] = ruleFn(value);

      if (!errors.value[field]) {
        errors.value[field] = {};
      }

      if (!isValid) {
        errors.value[field][ruleName] = errorMessage;
      } else {
        delete errors.value[field][ruleName];
      }
    }
  };

  return {
    errors,
    isValidForm,
    rules: {
      required,
    },
    validate(
      request: Record<string, any>,
      validations: Record<string, RuleFunction[]>
    ): void {
      Object.keys(validations).forEach((field) => {
        validateField(request[field], field, validations[field]);

        // Clean
        if (Object.keys(errors.value[field] || {}).length === 0) {
          delete errors.value[field];
        }
      });
    },
  };
}
