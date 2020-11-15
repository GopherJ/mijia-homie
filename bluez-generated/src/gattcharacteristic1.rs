// This code was autogenerated with `dbus-codegen-rust --file=specs/org.bluez.GattCharacteristic1.xml --interfaces=org.bluez.GattCharacteristic1 --client=nonblock --methodtype=none`, see https://github.com/diwic/dbus-rs
#[allow(unused_imports)]
use dbus::arg;
use dbus::nonblock;

pub trait OrgBluezGattCharacteristic1 {
    fn read_value(
        &self,
        options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> nonblock::MethodReply<Vec<u8>>;
    fn write_value(
        &self,
        value: Vec<u8>,
        options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> nonblock::MethodReply<()>;
    fn acquire_write(
        &self,
        options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> nonblock::MethodReply<(arg::OwnedFd, u16)>;
    fn acquire_notify(
        &self,
        options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> nonblock::MethodReply<(arg::OwnedFd, u16)>;
    fn start_notify(&self) -> nonblock::MethodReply<()>;
    fn stop_notify(&self) -> nonblock::MethodReply<()>;
    fn uuid(&self) -> nonblock::MethodReply<String>;
    fn service(&self) -> nonblock::MethodReply<dbus::Path<'static>>;
    fn value(&self) -> nonblock::MethodReply<Vec<u8>>;
    fn notifying(&self) -> nonblock::MethodReply<bool>;
    fn flags(&self) -> nonblock::MethodReply<Vec<String>>;
    fn write_acquired(&self) -> nonblock::MethodReply<bool>;
    fn notify_acquired(&self) -> nonblock::MethodReply<bool>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> OrgBluezGattCharacteristic1
    for nonblock::Proxy<'a, C>
{
    fn read_value(
        &self,
        options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> nonblock::MethodReply<Vec<u8>> {
        self.method_call("org.bluez.GattCharacteristic1", "ReadValue", (options,))
            .and_then(|r: (Vec<u8>,)| Ok(r.0))
    }

    fn write_value(
        &self,
        value: Vec<u8>,
        options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> nonblock::MethodReply<()> {
        self.method_call(
            "org.bluez.GattCharacteristic1",
            "WriteValue",
            (value, options),
        )
    }

    fn acquire_write(
        &self,
        options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> nonblock::MethodReply<(arg::OwnedFd, u16)> {
        self.method_call("org.bluez.GattCharacteristic1", "AcquireWrite", (options,))
    }

    fn acquire_notify(
        &self,
        options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    ) -> nonblock::MethodReply<(arg::OwnedFd, u16)> {
        self.method_call("org.bluez.GattCharacteristic1", "AcquireNotify", (options,))
    }

    fn start_notify(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.GattCharacteristic1", "StartNotify", ())
    }

    fn stop_notify(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.GattCharacteristic1", "StopNotify", ())
    }

    fn uuid(&self) -> nonblock::MethodReply<String> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "UUID",
        )
    }

    fn service(&self) -> nonblock::MethodReply<dbus::Path<'static>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "Service",
        )
    }

    fn value(&self) -> nonblock::MethodReply<Vec<u8>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "Value",
        )
    }

    fn notifying(&self) -> nonblock::MethodReply<bool> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "Notifying",
        )
    }

    fn flags(&self) -> nonblock::MethodReply<Vec<String>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "Flags",
        )
    }

    fn write_acquired(&self) -> nonblock::MethodReply<bool> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "WriteAcquired",
        )
    }

    fn notify_acquired(&self) -> nonblock::MethodReply<bool> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.bluez.GattCharacteristic1",
            "NotifyAcquired",
        )
    }
}
