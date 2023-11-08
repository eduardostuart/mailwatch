import type { Ref, ComputedRef } from "vue";
import { computed, ref } from "vue";

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
  errors: Ref<Record<string, FormErrors> | undefined>;

  /**
   * Current status of the form, if valid or not
   */
  isValidForm: ComputedRef<boolean>;

  /**
   * List of available rules
   */
  rules: Record<string, (...args: any) => RuleFunction>;

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

// Rule to check if a required field is empty or not
function isNumber(msg: string = "Field is not a number"): RuleFunction {
  return [
    "is-number",
    (input: any): RuleResponse => {
      if (!input) {
        return [false, msg];
      }
      return [/[0-9]/g.test(input), msg];
    },
  ];
}

function requiredIf(
  condition: boolean,
  msg: string = "Field is required"
): RuleFunction {
  return [
    "required-if",
    (input: any): RuleResponse => {
      if (condition) {
        return [input?.toString() !== "", msg];
      }
      return [true, msg];
    },
  ];
}

export function useFormValidation(): FormValidation {
  const errors = ref<Record<string, Record<string, string>> | undefined>(
    undefined
  );

  const isValidForm = computed(() => {
    if (!errors.value) {
      return false;
    }
    return Object.keys(errors.value).length === 0;
  });

  const validateField = (value: any, field: string, rules: RuleFunction[]) => {
    if (!errors.value) {
      errors.value = {};
    }
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
      isNumber,
      requiredIf,
    },
    validate(
      request: Record<string, any>,
      validations: Record<string, RuleFunction[]>
    ): void {
      Object.keys(validations).forEach((field) => {
        validateField(request[field], field, validations[field]);

        if (!errors.value) {
          errors.value = {};
        }

        // Clean
        if (Object.keys(errors.value[field] || {}).length === 0) {
          delete errors.value[field];
        }
      });
    },
  };
}
