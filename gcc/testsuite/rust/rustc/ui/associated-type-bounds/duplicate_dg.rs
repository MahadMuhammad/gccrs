#![feature(type_alias_impl_trait)]

use std::iter;
use std::mem::ManuallyDrop;

struct SI1<T: Iterator<Item: Copy, Item: Send>> {
// { dg-error ".E0719." "" { target *-*-* } .-1 }
    f: T,
}
struct SI2<T: Iterator<Item: Copy, Item: Copy>> {
// { dg-error ".E0719." "" { target *-*-* } .-1 }
    f: T,
}
struct SI3<T: Iterator<Item: 'static, Item: 'static>> {
// { dg-error ".E0719." "" { target *-*-* } .-1 }
    f: T,
}
struct SW1<T>
where
    T: Iterator<Item: Copy, Item: Send>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
    f: T,
}
struct SW2<T>
where
    T: Iterator<Item: Copy, Item: Copy>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
    f: T,
}
struct SW3<T>
where
    T: Iterator<Item: 'static, Item: 'static>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
    f: T,
}

enum EI1<T: Iterator<Item: Copy, Item: Send>> {
// { dg-error ".E0719." "" { target *-*-* } .-1 }
    V(T),
}
enum EI2<T: Iterator<Item: Copy, Item: Copy>> {
// { dg-error ".E0719." "" { target *-*-* } .-1 }
    V(T),
}
enum EI3<T: Iterator<Item: 'static, Item: 'static>> {
// { dg-error ".E0719." "" { target *-*-* } .-1 }
    V(T),
}
enum EW1<T>
where
    T: Iterator<Item: Copy, Item: Send>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
    V(T),
}
enum EW2<T>
where
    T: Iterator<Item: Copy, Item: Copy>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
    V(T),
}
enum EW3<T>
where
    T: Iterator<Item: 'static, Item: 'static>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
    V(T),
}

union UI1<T: Iterator<Item: Copy, Item: Send>> {
// { dg-error ".E0719." "" { target *-*-* } .-1 }
    f: ManuallyDrop<T>,
}
union UI2<T: Iterator<Item: Copy, Item: Copy>> {
// { dg-error ".E0719." "" { target *-*-* } .-1 }
    f: ManuallyDrop<T>,
}
union UI3<T: Iterator<Item: 'static, Item: 'static>> {
// { dg-error ".E0719." "" { target *-*-* } .-1 }
    f: ManuallyDrop<T>,
}
union UW1<T>
where
    T: Iterator<Item: Copy, Item: Send>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
    f: ManuallyDrop<T>,
}
union UW2<T>
where
    T: Iterator<Item: Copy, Item: Copy>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
    f: ManuallyDrop<T>,
}
union UW3<T>
where
    T: Iterator<Item: 'static, Item: 'static>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
    f: ManuallyDrop<T>,
}

fn FI1<T: Iterator<Item: Copy, Item: Send>>() {}
// { dg-error ".E0719." "" { target *-*-* } .-1 }
fn FI2<T: Iterator<Item: Copy, Item: Copy>>() {}
// { dg-error ".E0719." "" { target *-*-* } .-1 }
fn FI3<T: Iterator<Item: 'static, Item: 'static>>() {}
// { dg-error ".E0719." "" { target *-*-* } .-1 }
fn FW1<T>()
where
    T: Iterator<Item: Copy, Item: Send>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
}
fn FW2<T>()
where
    T: Iterator<Item: Copy, Item: Copy>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
}
fn FW3<T>()
where
    T: Iterator<Item: 'static, Item: 'static>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
}

fn FRPIT1() -> impl Iterator<Item: Copy, Item: Send> {
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
    iter::empty()
// { dg-error ".E0282." "" { target *-*-* } .-1 }
}
fn FRPIT2() -> impl Iterator<Item: Copy, Item: Copy> {
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
    iter::empty()
// { dg-error ".E0282." "" { target *-*-* } .-1 }
}
fn FRPIT3() -> impl Iterator<Item: 'static, Item: 'static> {
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
    iter::empty()
// { dg-error ".E0282." "" { target *-*-* } .-1 }
}
fn FAPIT1(_: impl Iterator<Item: Copy, Item: Send>) {}
// { dg-error ".E0719." "" { target *-*-* } .-1 }
fn FAPIT2(_: impl Iterator<Item: Copy, Item: Copy>) {}
// { dg-error ".E0719." "" { target *-*-* } .-1 }
fn FAPIT3(_: impl Iterator<Item: 'static, Item: 'static>) {}
// { dg-error ".E0719." "" { target *-*-* } .-1 }

type TAI1<T: Iterator<Item: Copy, Item: Send>> = T;
// { dg-error ".E0719." "" { target *-*-* } .-1 }
type TAI2<T: Iterator<Item: Copy, Item: Copy>> = T;
// { dg-error ".E0719." "" { target *-*-* } .-1 }
type TAI3<T: Iterator<Item: 'static, Item: 'static>> = T;
// { dg-error ".E0719." "" { target *-*-* } .-1 }
type TAW1<T>
where
    T: Iterator<Item: Copy, Item: Send>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
= T;
type TAW2<T>
where
    T: Iterator<Item: Copy, Item: Copy>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
= T;
type TAW3<T>
where
    T: Iterator<Item: 'static, Item: 'static>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
= T;

type ETAI1<T: Iterator<Item: Copy, Item: Send>> = impl Copy;
// { dg-error ".E0719." "" { target *-*-* } .-1 }
type ETAI2<T: Iterator<Item: Copy, Item: Copy>> = impl Copy;
// { dg-error ".E0719." "" { target *-*-* } .-1 }
type ETAI3<T: Iterator<Item: 'static, Item: 'static>> = impl Copy;
// { dg-error ".E0719." "" { target *-*-* } .-1 }
type ETAI4 = impl Iterator<Item: Copy, Item: Send>;
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
type ETAI5 = impl Iterator<Item: Copy, Item: Copy>;
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
type ETAI6 = impl Iterator<Item: 'static, Item: 'static>;
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }

trait TRI1<T: Iterator<Item: Copy, Item: Send>> {}
// { dg-error ".E0719." "" { target *-*-* } .-1 }
trait TRI2<T: Iterator<Item: Copy, Item: Copy>> {}
// { dg-error ".E0719." "" { target *-*-* } .-1 }
trait TRI3<T: Iterator<Item: 'static, Item: 'static>> {}
// { dg-error ".E0719." "" { target *-*-* } .-1 }
trait TRS1: Iterator<Item: Copy, Item: Send> {}
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
// { dg-error ".E0719." "" { target *-*-* } .-3 }
trait TRS2: Iterator<Item: Copy, Item: Copy> {}
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
// { dg-error ".E0719." "" { target *-*-* } .-3 }
trait TRS3: Iterator<Item: 'static, Item: 'static> {}
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
// { dg-error ".E0719." "" { target *-*-* } .-3 }
trait TRW1<T>
where
    T: Iterator<Item: Copy, Item: Send>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
}
trait TRW2<T>
where
    T: Iterator<Item: Copy, Item: Copy>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
}
trait TRW3<T>
where
    T: Iterator<Item: 'static, Item: 'static>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
{
}
trait TRSW1
where
    Self: Iterator<Item: Copy, Item: Send>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
// { dg-error ".E0719." "" { target *-*-* } .-3 }
{
}
trait TRSW2
where
    Self: Iterator<Item: Copy, Item: Copy>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
// { dg-error ".E0719." "" { target *-*-* } .-3 }
{
}
trait TRSW3
where
    Self: Iterator<Item: 'static, Item: 'static>,
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
// { dg-error ".E0719." "" { target *-*-* } .-3 }
{
}
trait TRA1 {
    type A: Iterator<Item: Copy, Item: Send>;
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
}
trait TRA2 {
    type A: Iterator<Item: Copy, Item: Copy>;
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
}
trait TRA3 {
    type A: Iterator<Item: 'static, Item: 'static>;
// { dg-error ".E0719." "" { target *-*-* } .-1 }
// { dg-error ".E0719." "" { target *-*-* } .-2 }
}

fn main() {}

