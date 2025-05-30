// automatically generated by the FlatBuffers compiler, do not modify

// @generated

// use core::cmp::Ordering;
// use core::mem;

extern crate flatbuffers;
// use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod atrimo {

    use core::cmp::Ordering;
    use core::mem;

    extern crate flatbuffers;
    use self::flatbuffers::{EndianScalar, Follow};
    #[allow(unused_imports, dead_code)]
    pub mod snapshot_events {

        use core::cmp::Ordering;
        use core::mem;

        extern crate flatbuffers;
        use self::flatbuffers::{EndianScalar, Follow};

        // struct SnapshotAskData, aligned to 8
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Default)]
        pub struct SnapshotAskData(pub [u8; 16]);

        impl core::fmt::Debug for SnapshotAskData {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SnapshotAskData")
                    .field("price", &self.price())
                    .field("qty", &self.qty())
                    .finish()
            }
        }

        impl flatbuffers::SimpleToVerifyInSlice for SnapshotAskData {}
        impl<'a> flatbuffers::Follow<'a> for SnapshotAskData {
            type Inner = &'a SnapshotAskData;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                <&'a SnapshotAskData>::follow(buf, loc)
            }
        }
        impl<'a> flatbuffers::Follow<'a> for &'a SnapshotAskData {
            type Inner = &'a SnapshotAskData;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                flatbuffers::follow_cast_ref::<SnapshotAskData>(buf, loc)
            }
        }
        impl flatbuffers::Push for SnapshotAskData {
            type Output = SnapshotAskData;
            #[inline]
            unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
                let src = ::core::slice::from_raw_parts(
                    self as *const SnapshotAskData as *const u8,
                    Self::size(),
                );
                dst.copy_from_slice(src);
            }
        }

        impl flatbuffers::Verifiable for SnapshotAskData {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.in_buffer::<Self>(pos)
            }
        }

        impl SnapshotAskData {
            #[allow(clippy::too_many_arguments)]
            pub fn new(price: u64, qty: u64) -> Self {
                let mut s = Self([0; 16]);
                s.set_price(price);
                s.set_qty(qty);
                s
            }

            pub fn price(&self) -> u64 {
                let mut mem = core::mem::MaybeUninit::<<u64 as EndianScalar>::Scalar>::uninit();
                // Safety:
                // Created from a valid Table for this object
                // Which contains a valid value in this slot
                EndianScalar::from_little_endian(unsafe {
                    core::ptr::copy_nonoverlapping(
                        self.0[0..].as_ptr(),
                        mem.as_mut_ptr() as *mut u8,
                        core::mem::size_of::<<u64 as EndianScalar>::Scalar>(),
                    );
                    mem.assume_init()
                })
            }

            pub fn set_price(&mut self, x: u64) {
                let x_le = x.to_little_endian();
                // Safety:
                // Created from a valid Table for this object
                // Which contains a valid value in this slot
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        &x_le as *const _ as *const u8,
                        self.0[0..].as_mut_ptr(),
                        core::mem::size_of::<<u64 as EndianScalar>::Scalar>(),
                    );
                }
            }

            pub fn qty(&self) -> u64 {
                let mut mem = core::mem::MaybeUninit::<<u64 as EndianScalar>::Scalar>::uninit();
                // Safety:
                // Created from a valid Table for this object
                // Which contains a valid value in this slot
                EndianScalar::from_little_endian(unsafe {
                    core::ptr::copy_nonoverlapping(
                        self.0[8..].as_ptr(),
                        mem.as_mut_ptr() as *mut u8,
                        core::mem::size_of::<<u64 as EndianScalar>::Scalar>(),
                    );
                    mem.assume_init()
                })
            }

            pub fn set_qty(&mut self, x: u64) {
                let x_le = x.to_little_endian();
                // Safety:
                // Created from a valid Table for this object
                // Which contains a valid value in this slot
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        &x_le as *const _ as *const u8,
                        self.0[8..].as_mut_ptr(),
                        core::mem::size_of::<<u64 as EndianScalar>::Scalar>(),
                    );
                }
            }
        }

        // struct SnapshotBidData, aligned to 8
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Default)]
        pub struct SnapshotBidData(pub [u8; 16]);

        impl core::fmt::Debug for SnapshotBidData {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("SnapshotBidData")
                    .field("price", &self.price())
                    .field("qty", &self.qty())
                    .finish()
            }
        }

        impl flatbuffers::SimpleToVerifyInSlice for SnapshotBidData {}
        impl<'a> flatbuffers::Follow<'a> for SnapshotBidData {
            type Inner = &'a SnapshotBidData;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                <&'a SnapshotBidData>::follow(buf, loc)
            }
        }
        impl<'a> flatbuffers::Follow<'a> for &'a SnapshotBidData {
            type Inner = &'a SnapshotBidData;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                flatbuffers::follow_cast_ref::<SnapshotBidData>(buf, loc)
            }
        }
        impl flatbuffers::Push for SnapshotBidData {
            type Output = SnapshotBidData;
            #[inline]
            unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
                let src = ::core::slice::from_raw_parts(
                    self as *const SnapshotBidData as *const u8,
                    Self::size(),
                );
                dst.copy_from_slice(src);
            }
        }

        impl flatbuffers::Verifiable for SnapshotBidData {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.in_buffer::<Self>(pos)
            }
        }

        impl SnapshotBidData {
            #[allow(clippy::too_many_arguments)]
            pub fn new(price: u64, qty: u64) -> Self {
                let mut s = Self([0; 16]);
                s.set_price(price);
                s.set_qty(qty);
                s
            }

            pub fn price(&self) -> u64 {
                let mut mem = core::mem::MaybeUninit::<<u64 as EndianScalar>::Scalar>::uninit();
                // Safety:
                // Created from a valid Table for this object
                // Which contains a valid value in this slot
                EndianScalar::from_little_endian(unsafe {
                    core::ptr::copy_nonoverlapping(
                        self.0[0..].as_ptr(),
                        mem.as_mut_ptr() as *mut u8,
                        core::mem::size_of::<<u64 as EndianScalar>::Scalar>(),
                    );
                    mem.assume_init()
                })
            }

            pub fn set_price(&mut self, x: u64) {
                let x_le = x.to_little_endian();
                // Safety:
                // Created from a valid Table for this object
                // Which contains a valid value in this slot
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        &x_le as *const _ as *const u8,
                        self.0[0..].as_mut_ptr(),
                        core::mem::size_of::<<u64 as EndianScalar>::Scalar>(),
                    );
                }
            }

            pub fn qty(&self) -> u64 {
                let mut mem = core::mem::MaybeUninit::<<u64 as EndianScalar>::Scalar>::uninit();
                // Safety:
                // Created from a valid Table for this object
                // Which contains a valid value in this slot
                EndianScalar::from_little_endian(unsafe {
                    core::ptr::copy_nonoverlapping(
                        self.0[8..].as_ptr(),
                        mem.as_mut_ptr() as *mut u8,
                        core::mem::size_of::<<u64 as EndianScalar>::Scalar>(),
                    );
                    mem.assume_init()
                })
            }

            pub fn set_qty(&mut self, x: u64) {
                let x_le = x.to_little_endian();
                // Safety:
                // Created from a valid Table for this object
                // Which contains a valid value in this slot
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        &x_le as *const _ as *const u8,
                        self.0[8..].as_mut_ptr(),
                        core::mem::size_of::<<u64 as EndianScalar>::Scalar>(),
                    );
                }
            }
        }

        pub enum SnapshotDataOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct SnapshotData<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for SnapshotData<'a> {
            type Inner = SnapshotData<'a>;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table::new(buf, loc),
                }
            }
        }

        impl<'a> SnapshotData<'a> {
            pub const VT_ASKS: flatbuffers::VOffsetT = 4;
            pub const VT_BIDS: flatbuffers::VOffsetT = 6;

            #[inline]
            pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                SnapshotData { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args SnapshotDataArgs<'args>,
            ) -> flatbuffers::WIPOffset<SnapshotData<'bldr>> {
                let mut builder = SnapshotDataBuilder::new(_fbb);
                if let Some(x) = args.bids {
                    builder.add_bids(x);
                }
                if let Some(x) = args.asks {
                    builder.add_asks(x);
                }
                builder.finish()
            }

            #[inline]
            pub fn asks(&self) -> Option<flatbuffers::Vector<'a, SnapshotAskData>> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, SnapshotAskData>>>(SnapshotData::VT_ASKS, None)
                }
            }
            #[inline]
            pub fn bids(&self) -> Option<flatbuffers::Vector<'a, SnapshotBidData>> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, SnapshotBidData>>>(SnapshotData::VT_BIDS, None)
                }
            }
        }

        impl flatbuffers::Verifiable for SnapshotData<'_> {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, SnapshotAskData>>>("asks", Self::VT_ASKS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, SnapshotBidData>>>("bids", Self::VT_BIDS, false)?
     .finish();
                Ok(())
            }
        }
        pub struct SnapshotDataArgs<'a> {
            pub asks: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, SnapshotAskData>>>,
            pub bids: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, SnapshotBidData>>>,
        }
        impl<'a> Default for SnapshotDataArgs<'a> {
            #[inline]
            fn default() -> Self {
                SnapshotDataArgs {
                    asks: None,
                    bids: None,
                }
            }
        }

        pub struct SnapshotDataBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> SnapshotDataBuilder<'a, 'b> {
            #[inline]
            pub fn add_asks(
                &mut self,
                asks: flatbuffers::WIPOffset<flatbuffers::Vector<'b, SnapshotAskData>>,
            ) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(SnapshotData::VT_ASKS, asks);
            }
            #[inline]
            pub fn add_bids(
                &mut self,
                bids: flatbuffers::WIPOffset<flatbuffers::Vector<'b, SnapshotBidData>>,
            ) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(SnapshotData::VT_BIDS, bids);
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            ) -> SnapshotDataBuilder<'a, 'b> {
                let start = _fbb.start_table();
                SnapshotDataBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<SnapshotData<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl core::fmt::Debug for SnapshotData<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut ds = f.debug_struct("SnapshotData");
                ds.field("asks", &self.asks());
                ds.field("bids", &self.bids());
                ds.finish()
            }
        }
        pub enum SnapshotEventOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct SnapshotEvent<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for SnapshotEvent<'a> {
            type Inner = SnapshotEvent<'a>;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table::new(buf, loc),
                }
            }
        }

        impl<'a> SnapshotEvent<'a> {
            pub const VT_EXCHANGE: flatbuffers::VOffsetT = 4;
            pub const VT_INSTRUMENT: flatbuffers::VOffsetT = 6;
            pub const VT_TIMESTAMP: flatbuffers::VOffsetT = 8;
            pub const VT_SNAPSHOT: flatbuffers::VOffsetT = 10;

            #[inline]
            pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                SnapshotEvent { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args SnapshotEventArgs<'args>,
            ) -> flatbuffers::WIPOffset<SnapshotEvent<'bldr>> {
                let mut builder = SnapshotEventBuilder::new(_fbb);
                builder.add_timestamp(args.timestamp);
                if let Some(x) = args.snapshot {
                    builder.add_snapshot(x);
                }
                if let Some(x) = args.instrument {
                    builder.add_instrument(x);
                }
                if let Some(x) = args.exchange {
                    builder.add_exchange(x);
                }
                builder.finish()
            }

            #[inline]
            pub fn exchange(&self) -> Option<&'a str> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<flatbuffers::ForwardsUOffset<&str>>(SnapshotEvent::VT_EXCHANGE, None)
                }
            }
            #[inline]
            pub fn instrument(&self) -> Option<&'a str> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(
                        SnapshotEvent::VT_INSTRUMENT,
                        None,
                    )
                }
            }
            #[inline]
            pub fn timestamp(&self) -> u64 {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<u64>(SnapshotEvent::VT_TIMESTAMP, Some(0))
                        .unwrap()
                }
            }
            #[inline]
            pub fn snapshot(&self) -> Option<SnapshotData<'a>> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab.get::<flatbuffers::ForwardsUOffset<SnapshotData>>(
                        SnapshotEvent::VT_SNAPSHOT,
                        None,
                    )
                }
            }
        }

        impl flatbuffers::Verifiable for SnapshotEvent<'_> {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.visit_table(pos)?
                    .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                        "exchange",
                        Self::VT_EXCHANGE,
                        false,
                    )?
                    .visit_field::<flatbuffers::ForwardsUOffset<&str>>(
                        "instrument",
                        Self::VT_INSTRUMENT,
                        false,
                    )?
                    .visit_field::<u64>("timestamp", Self::VT_TIMESTAMP, false)?
                    .visit_field::<flatbuffers::ForwardsUOffset<SnapshotData>>(
                        "snapshot",
                        Self::VT_SNAPSHOT,
                        false,
                    )?
                    .finish();
                Ok(())
            }
        }
        pub struct SnapshotEventArgs<'a> {
            pub exchange: Option<flatbuffers::WIPOffset<&'a str>>,
            pub instrument: Option<flatbuffers::WIPOffset<&'a str>>,
            pub timestamp: u64,
            pub snapshot: Option<flatbuffers::WIPOffset<SnapshotData<'a>>>,
        }
        impl<'a> Default for SnapshotEventArgs<'a> {
            #[inline]
            fn default() -> Self {
                SnapshotEventArgs {
                    exchange: None,
                    instrument: None,
                    timestamp: 0,
                    snapshot: None,
                }
            }
        }

        pub struct SnapshotEventBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> SnapshotEventBuilder<'a, 'b> {
            #[inline]
            pub fn add_exchange(&mut self, exchange: flatbuffers::WIPOffset<&'b str>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                    SnapshotEvent::VT_EXCHANGE,
                    exchange,
                );
            }
            #[inline]
            pub fn add_instrument(&mut self, instrument: flatbuffers::WIPOffset<&'b str>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                    SnapshotEvent::VT_INSTRUMENT,
                    instrument,
                );
            }
            #[inline]
            pub fn add_timestamp(&mut self, timestamp: u64) {
                self.fbb_
                    .push_slot::<u64>(SnapshotEvent::VT_TIMESTAMP, timestamp, 0);
            }
            #[inline]
            pub fn add_snapshot(&mut self, snapshot: flatbuffers::WIPOffset<SnapshotData<'b>>) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<SnapshotData>>(
                        SnapshotEvent::VT_SNAPSHOT,
                        snapshot,
                    );
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            ) -> SnapshotEventBuilder<'a, 'b> {
                let start = _fbb.start_table();
                SnapshotEventBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<SnapshotEvent<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl core::fmt::Debug for SnapshotEvent<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut ds = f.debug_struct("SnapshotEvent");
                ds.field("exchange", &self.exchange());
                ds.field("instrument", &self.instrument());
                ds.field("timestamp", &self.timestamp());
                ds.field("snapshot", &self.snapshot());
                ds.finish()
            }
        }
        pub enum SnapshotEventMessageOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct SnapshotEventMessage<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for SnapshotEventMessage<'a> {
            type Inner = SnapshotEventMessage<'a>;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table::new(buf, loc),
                }
            }
        }

        impl<'a> SnapshotEventMessage<'a> {
            pub const VT_SNAPSHOT_EVENT: flatbuffers::VOffsetT = 4;
            pub const VT_MESSAGE_TYPE: flatbuffers::VOffsetT = 6;

            #[inline]
            pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                SnapshotEventMessage { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args SnapshotEventMessageArgs<'args>,
            ) -> flatbuffers::WIPOffset<SnapshotEventMessage<'bldr>> {
                let mut builder = SnapshotEventMessageBuilder::new(_fbb);
                builder.add_message_type(args.message_type);
                if let Some(x) = args.snapshot_event {
                    builder.add_snapshot_event(x);
                }
                builder.finish()
            }

            #[inline]
            pub fn snapshot_event(&self) -> Option<SnapshotEvent<'a>> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<flatbuffers::ForwardsUOffset<SnapshotEvent>>(
                            SnapshotEventMessage::VT_SNAPSHOT_EVENT,
                            None,
                        )
                }
            }
            #[inline]
            pub fn message_type(&self) -> i32 {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<i32>(SnapshotEventMessage::VT_MESSAGE_TYPE, Some(1))
                        .unwrap()
                }
            }
        }

        impl flatbuffers::Verifiable for SnapshotEventMessage<'_> {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.visit_table(pos)?
                    .visit_field::<flatbuffers::ForwardsUOffset<SnapshotEvent>>(
                        "snapshot_event",
                        Self::VT_SNAPSHOT_EVENT,
                        false,
                    )?
                    .visit_field::<i32>("message_type", Self::VT_MESSAGE_TYPE, false)?
                    .finish();
                Ok(())
            }
        }
        pub struct SnapshotEventMessageArgs<'a> {
            pub snapshot_event: Option<flatbuffers::WIPOffset<SnapshotEvent<'a>>>,
            pub message_type: i32,
        }
        impl<'a> Default for SnapshotEventMessageArgs<'a> {
            #[inline]
            fn default() -> Self {
                SnapshotEventMessageArgs {
                    snapshot_event: None,
                    message_type: 1,
                }
            }
        }

        pub struct SnapshotEventMessageBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> SnapshotEventMessageBuilder<'a, 'b> {
            #[inline]
            pub fn add_snapshot_event(
                &mut self,
                snapshot_event: flatbuffers::WIPOffset<SnapshotEvent<'b>>,
            ) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<SnapshotEvent>>(
                        SnapshotEventMessage::VT_SNAPSHOT_EVENT,
                        snapshot_event,
                    );
            }
            #[inline]
            pub fn add_message_type(&mut self, message_type: i32) {
                self.fbb_
                    .push_slot::<i32>(SnapshotEventMessage::VT_MESSAGE_TYPE, message_type, 1);
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            ) -> SnapshotEventMessageBuilder<'a, 'b> {
                let start = _fbb.start_table();
                SnapshotEventMessageBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<SnapshotEventMessage<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl core::fmt::Debug for SnapshotEventMessage<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut ds = f.debug_struct("SnapshotEventMessage");
                ds.field("snapshot_event", &self.snapshot_event());
                ds.field("message_type", &self.message_type());
                ds.finish()
            }
        }
        #[inline]
        /// Verifies that a buffer of bytes contains a `SnapshotEventMessage`
        /// and returns it.
        /// Note that verification is still experimental and may not
        /// catch every error, or be maximally performant. For the
        /// previous, unchecked, behavior use
        /// `root_as_snapshot_event_message_unchecked`.
        pub fn root_as_snapshot_event_message(
            buf: &[u8],
        ) -> Result<SnapshotEventMessage, flatbuffers::InvalidFlatbuffer> {
            flatbuffers::root::<SnapshotEventMessage>(buf)
        }
        #[inline]
        /// Verifies that a buffer of bytes contains a size prefixed
        /// `SnapshotEventMessage` and returns it.
        /// Note that verification is still experimental and may not
        /// catch every error, or be maximally performant. For the
        /// previous, unchecked, behavior use
        /// `size_prefixed_root_as_snapshot_event_message_unchecked`.
        pub fn size_prefixed_root_as_snapshot_event_message(
            buf: &[u8],
        ) -> Result<SnapshotEventMessage, flatbuffers::InvalidFlatbuffer> {
            flatbuffers::size_prefixed_root::<SnapshotEventMessage>(buf)
        }
        #[inline]
        /// Verifies, with the given options, that a buffer of bytes
        /// contains a `SnapshotEventMessage` and returns it.
        /// Note that verification is still experimental and may not
        /// catch every error, or be maximally performant. For the
        /// previous, unchecked, behavior use
        /// `root_as_snapshot_event_message_unchecked`.
        pub fn root_as_snapshot_event_message_with_opts<'b, 'o>(
            opts: &'o flatbuffers::VerifierOptions,
            buf: &'b [u8],
        ) -> Result<SnapshotEventMessage<'b>, flatbuffers::InvalidFlatbuffer> {
            flatbuffers::root_with_opts::<SnapshotEventMessage<'b>>(opts, buf)
        }
        #[inline]
        /// Verifies, with the given verifier options, that a buffer of
        /// bytes contains a size prefixed `SnapshotEventMessage` and returns
        /// it. Note that verification is still experimental and may not
        /// catch every error, or be maximally performant. For the
        /// previous, unchecked, behavior use
        /// `root_as_snapshot_event_message_unchecked`.
        pub fn size_prefixed_root_as_snapshot_event_message_with_opts<'b, 'o>(
            opts: &'o flatbuffers::VerifierOptions,
            buf: &'b [u8],
        ) -> Result<SnapshotEventMessage<'b>, flatbuffers::InvalidFlatbuffer> {
            flatbuffers::size_prefixed_root_with_opts::<SnapshotEventMessage<'b>>(opts, buf)
        }
        #[inline]
        /// Assumes, without verification, that a buffer of bytes contains a SnapshotEventMessage and returns it.
        /// # Safety
        /// Callers must trust the given bytes do indeed contain a valid `SnapshotEventMessage`.
        pub unsafe fn root_as_snapshot_event_message_unchecked(buf: &[u8]) -> SnapshotEventMessage {
            flatbuffers::root_unchecked::<SnapshotEventMessage>(buf)
        }
        #[inline]
        /// Assumes, without verification, that a buffer of bytes contains a size prefixed SnapshotEventMessage and returns it.
        /// # Safety
        /// Callers must trust the given bytes do indeed contain a valid size prefixed `SnapshotEventMessage`.
        pub unsafe fn size_prefixed_root_as_snapshot_event_message_unchecked(
            buf: &[u8],
        ) -> SnapshotEventMessage {
            flatbuffers::size_prefixed_root_unchecked::<SnapshotEventMessage>(buf)
        }
        #[inline]
        pub fn finish_snapshot_event_message_buffer<'a>(
            fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<SnapshotEventMessage<'a>>,
        ) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_snapshot_event_message_buffer<'a>(
            fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<SnapshotEventMessage<'a>>,
        ) {
            fbb.finish_size_prefixed(root, None);
        }
    } // pub mod SnapshotEvents
} // pub mod Atrimo
