"use client";
import Button from "@/components/Button";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import Text from "@/components/Text";
import View from "@/components/View";
import { Checkbox, Form, FormProps, Input } from "antd";

type FormFieldTypes = {
  firstName?: string;
  lastName?: string;
  email?: string;
  password?: string;
  acceptEULA?: boolean;
};

export default function Page() {
  const [form] = Form.useForm();
  const submitForm: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    console.log("Success:", { ...values });
  };
  const submitFormFailed: FormProps<FormFieldTypes>["onFinishFailed"] = (
    errorInfo
  ) => {
    console.log("Failed:", errorInfo);
  };
  return (
    <View className="h-screen grid grid-cols-12 bg-gray-50 gap-x-12">
      <View className="col-span-5 py-8 px-6">
        <Heading className="font-semibold">Welcome!</Heading>
        <Text>Create a account to begin</Text>

        <Form
          initialValues={{ remember: true }}
          onFinish={submitForm}
          onFinishFailed={submitFormFailed}
          autoComplete="off"
          name="save-data"
          layout="vertical"
          className="my-4 mt-12 flex flex-col gap-2"
          form={form}
        >
          <Form.Item<FormFieldTypes>
            label="First name"
            name="firstName"
            rules={[{ required: true, message: "Please input the title!" }]}
          >
            <Input
              autoFocus
              type="text"
              name="firstName"
              className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
            />
          </Form.Item>
          <Form.Item<FormFieldTypes>
            label="Last name"
            name="lastName"
            rules={[{ required: true, message: "Please input the title!" }]}
          >
            <Input
              autoFocus
              type="text"
              name="title"
              className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
            />
          </Form.Item>
          <Form.Item<FormFieldTypes>
            label="Email"
            name="email"
            rules={[{ required: true, message: "Please input the title!" }]}
          >
            <Input
              autoFocus
              type="email"
              name="email"
              className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
            />
          </Form.Item>
          <Form.Item<FormFieldTypes>
            label="Password"
            name="password"
            rules={[{ required: true, message: "Please input the title!" }]}
          >
            <Input
              autoFocus
              type="password"
              name="password"
              className="w-full rounded-lg py-3 focus:border-app-500 focus:outline-none border bg-white border-gray-300 block placeholder:pb-2 px-2 "
            />
          </Form.Item>
          <View className="flex gap-2 my-2">
            <Checkbox /> <SmallText>I agree to the terms and privacy</SmallText>
          </View>
          <Button href="/dashboard" className=" bg-app-600 text-center w-full py-2 text-white">
            Sign up
          </Button>
        </Form>
        <SmallText>
          Already have an account?{" "}
          <a className="text-app-600" href="/authentication/login">
            Sign in
          </a>
        </SmallText>
      </View>
      <View className=" rounded-2xl shadow-sm m-6 col-span-7 shadow-gray-500 bg-app-600 bg-blend-multiply"></View>
    </View>
  );
}
