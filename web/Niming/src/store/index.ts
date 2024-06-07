import { defineStore } from "pinia";

export enum VerifyType {
  not,
  email,
  totp,
}

export interface User {
  name: string | null;
  password: string | null;
  verified: boolean;
  school_id: string | null;
  verify_type: VerifyType | null;
}

export const useUserStore = defineStore("user", {
  state: () => ({
    name: "",
    password: "",
    verified: false,
    school_id: "",
    verify_type: VerifyType.not,
  }),
  getters: {
    debug(state) {
      return {
        name: state.name,
        passwd: state.password,
        verified: state.verified,
        type: state.verify_type,
        id: state.school_id,
      };
    },
  },
  actions: {
    assign_name(name: string) {
      this.name = name;
    },
    assign_password(passwd: string) {
      this.password = passwd;
    },
    assign_verify_type(type: VerifyType) {
      this.verify_type = type;
    },
  },
});
