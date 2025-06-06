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
    pub mod update_events {

        use core::cmp::Ordering;
        use core::mem;

        extern crate flatbuffers;
        use self::flatbuffers::{EndianScalar, Follow};

        // struct UpdateAskData, aligned to 8
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Default)]
        pub struct UpdateAskData(pub [u8; 16]);

        impl core::fmt::Debug for UpdateAskData {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("UpdateAskData")
                    .field("price", &self.price())
                    .field("qty", &self.qty())
                    .finish()
            }
        }

        impl flatbuffers::SimpleToVerifyInSlice for UpdateAskData {}
        impl<'a> flatbuffers::Follow<'a> for UpdateAskData {
            type Inner = &'a UpdateAskData;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                <&'a UpdateAskData>::follow(buf, loc)
            }
        }
        impl<'a> flatbuffers::Follow<'a> for &'a UpdateAskData {
            type Inner = &'a UpdateAskData;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                flatbuffers::follow_cast_ref::<UpdateAskData>(buf, loc)
            }
        }
        impl flatbuffers::Push for UpdateAskData {
            type Output = UpdateAskData;
            #[inline]
            unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
                let src = ::core::slice::from_raw_parts(
                    self as *const UpdateAskData as *const u8,
                    Self::size(),
                );
                dst.copy_from_slice(src);
            }
        }

        impl flatbuffers::Verifiable for UpdateAskData {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.in_buffer::<Self>(pos)
            }
        }

        impl UpdateAskData {
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

        // struct UpdateBidData, aligned to 8
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Default)]
        pub struct UpdateBidData(pub [u8; 16]);

        impl core::fmt::Debug for UpdateBidData {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct("UpdateBidData")
                    .field("price", &self.price())
                    .field("qty", &self.qty())
                    .finish()
            }
        }

        impl flatbuffers::SimpleToVerifyInSlice for UpdateBidData {}
        impl<'a> flatbuffers::Follow<'a> for UpdateBidData {
            type Inner = &'a UpdateBidData;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                <&'a UpdateBidData>::follow(buf, loc)
            }
        }
        impl<'a> flatbuffers::Follow<'a> for &'a UpdateBidData {
            type Inner = &'a UpdateBidData;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                flatbuffers::follow_cast_ref::<UpdateBidData>(buf, loc)
            }
        }
        impl flatbuffers::Push for UpdateBidData {
            type Output = UpdateBidData;
            #[inline]
            unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
                let src = ::core::slice::from_raw_parts(
                    self as *const UpdateBidData as *const u8,
                    Self::size(),
                );
                dst.copy_from_slice(src);
            }
        }

        impl flatbuffers::Verifiable for UpdateBidData {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.in_buffer::<Self>(pos)
            }
        }

        impl UpdateBidData {
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

        pub enum UpdateDataOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct UpdateData<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for UpdateData<'a> {
            type Inner = UpdateData<'a>;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table::new(buf, loc),
                }
            }
        }

        impl<'a> UpdateData<'a> {
            pub const VT_ASKS: flatbuffers::VOffsetT = 4;
            pub const VT_BIDS: flatbuffers::VOffsetT = 6;

            #[inline]
            pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                UpdateData { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args UpdateDataArgs<'args>,
            ) -> flatbuffers::WIPOffset<UpdateData<'bldr>> {
                let mut builder = UpdateDataBuilder::new(_fbb);
                if let Some(x) = args.bids {
                    builder.add_bids(x);
                }
                if let Some(x) = args.asks {
                    builder.add_asks(x);
                }
                builder.finish()
            }

            #[inline]
            pub fn asks(&self) -> Option<flatbuffers::Vector<'a, UpdateAskData>> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, UpdateAskData>>>(UpdateData::VT_ASKS, None)
                }
            }
            #[inline]
            pub fn bids(&self) -> Option<flatbuffers::Vector<'a, UpdateBidData>> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, UpdateBidData>>>(UpdateData::VT_BIDS, None)
                }
            }
        }

        impl flatbuffers::Verifiable for UpdateData<'_> {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, UpdateAskData>>>("asks", Self::VT_ASKS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, UpdateBidData>>>("bids", Self::VT_BIDS, false)?
     .finish();
                Ok(())
            }
        }
        pub struct UpdateDataArgs<'a> {
            pub asks: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, UpdateAskData>>>,
            pub bids: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, UpdateBidData>>>,
        }
        impl<'a> Default for UpdateDataArgs<'a> {
            #[inline]
            fn default() -> Self {
                UpdateDataArgs {
                    asks: None,
                    bids: None,
                }
            }
        }

        pub struct UpdateDataBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> UpdateDataBuilder<'a, 'b> {
            #[inline]
            pub fn add_asks(
                &mut self,
                asks: flatbuffers::WIPOffset<flatbuffers::Vector<'b, UpdateAskData>>,
            ) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(UpdateData::VT_ASKS, asks);
            }
            #[inline]
            pub fn add_bids(
                &mut self,
                bids: flatbuffers::WIPOffset<flatbuffers::Vector<'b, UpdateBidData>>,
            ) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<_>>(UpdateData::VT_BIDS, bids);
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            ) -> UpdateDataBuilder<'a, 'b> {
                let start = _fbb.start_table();
                UpdateDataBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<UpdateData<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl core::fmt::Debug for UpdateData<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut ds = f.debug_struct("UpdateData");
                ds.field("asks", &self.asks());
                ds.field("bids", &self.bids());
                ds.finish()
            }
        }
        pub enum UpdateEventOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct UpdateEvent<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for UpdateEvent<'a> {
            type Inner = UpdateEvent<'a>;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table::new(buf, loc),
                }
            }
        }

        impl<'a> UpdateEvent<'a> {
            pub const VT_EXCHANGE: flatbuffers::VOffsetT = 4;
            pub const VT_INSTRUMENT: flatbuffers::VOffsetT = 6;
            pub const VT_TIMESTAMP: flatbuffers::VOffsetT = 8;
            pub const VT_UPDATE: flatbuffers::VOffsetT = 10;

            #[inline]
            pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                UpdateEvent { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args UpdateEventArgs<'args>,
            ) -> flatbuffers::WIPOffset<UpdateEvent<'bldr>> {
                let mut builder = UpdateEventBuilder::new(_fbb);
                builder.add_timestamp(args.timestamp);
                if let Some(x) = args.update {
                    builder.add_update(x);
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
                        .get::<flatbuffers::ForwardsUOffset<&str>>(UpdateEvent::VT_EXCHANGE, None)
                }
            }
            #[inline]
            pub fn instrument(&self) -> Option<&'a str> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<flatbuffers::ForwardsUOffset<&str>>(UpdateEvent::VT_INSTRUMENT, None)
                }
            }
            #[inline]
            pub fn timestamp(&self) -> u64 {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab
                        .get::<u64>(UpdateEvent::VT_TIMESTAMP, Some(0))
                        .unwrap()
                }
            }
            #[inline]
            pub fn update(&self) -> Option<UpdateData<'a>> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab.get::<flatbuffers::ForwardsUOffset<UpdateData>>(
                        UpdateEvent::VT_UPDATE,
                        None,
                    )
                }
            }
        }

        impl flatbuffers::Verifiable for UpdateEvent<'_> {
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
                    .visit_field::<flatbuffers::ForwardsUOffset<UpdateData>>(
                        "update",
                        Self::VT_UPDATE,
                        false,
                    )?
                    .finish();
                Ok(())
            }
        }
        pub struct UpdateEventArgs<'a> {
            pub exchange: Option<flatbuffers::WIPOffset<&'a str>>,
            pub instrument: Option<flatbuffers::WIPOffset<&'a str>>,
            pub timestamp: u64,
            pub update: Option<flatbuffers::WIPOffset<UpdateData<'a>>>,
        }
        impl<'a> Default for UpdateEventArgs<'a> {
            #[inline]
            fn default() -> Self {
                UpdateEventArgs {
                    exchange: None,
                    instrument: None,
                    timestamp: 0,
                    update: None,
                }
            }
        }

        pub struct UpdateEventBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> UpdateEventBuilder<'a, 'b> {
            #[inline]
            pub fn add_exchange(&mut self, exchange: flatbuffers::WIPOffset<&'b str>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                    UpdateEvent::VT_EXCHANGE,
                    exchange,
                );
            }
            #[inline]
            pub fn add_instrument(&mut self, instrument: flatbuffers::WIPOffset<&'b str>) {
                self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                    UpdateEvent::VT_INSTRUMENT,
                    instrument,
                );
            }
            #[inline]
            pub fn add_timestamp(&mut self, timestamp: u64) {
                self.fbb_
                    .push_slot::<u64>(UpdateEvent::VT_TIMESTAMP, timestamp, 0);
            }
            #[inline]
            pub fn add_update(&mut self, update: flatbuffers::WIPOffset<UpdateData<'b>>) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<UpdateData>>(
                        UpdateEvent::VT_UPDATE,
                        update,
                    );
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            ) -> UpdateEventBuilder<'a, 'b> {
                let start = _fbb.start_table();
                UpdateEventBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<UpdateEvent<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl core::fmt::Debug for UpdateEvent<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut ds = f.debug_struct("UpdateEvent");
                ds.field("exchange", &self.exchange());
                ds.field("instrument", &self.instrument());
                ds.field("timestamp", &self.timestamp());
                ds.field("update", &self.update());
                ds.finish()
            }
        }
        pub enum UpdateEventMessageOffset {}
        #[derive(Copy, Clone, PartialEq)]

        pub struct UpdateEventMessage<'a> {
            pub _tab: flatbuffers::Table<'a>,
        }

        impl<'a> flatbuffers::Follow<'a> for UpdateEventMessage<'a> {
            type Inner = UpdateEventMessage<'a>;
            #[inline]
            unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
                Self {
                    _tab: flatbuffers::Table::new(buf, loc),
                }
            }
        }

        impl<'a> UpdateEventMessage<'a> {
            pub const VT_UPDATE_EVENT: flatbuffers::VOffsetT = 4;
            pub const VT_MESSAGE_TYPE: flatbuffers::VOffsetT = 6;

            #[inline]
            pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
                UpdateEventMessage { _tab: table }
            }
            #[allow(unused_mut)]
            pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
                _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
                args: &'args UpdateEventMessageArgs<'args>,
            ) -> flatbuffers::WIPOffset<UpdateEventMessage<'bldr>> {
                let mut builder = UpdateEventMessageBuilder::new(_fbb);
                builder.add_message_type(args.message_type);
                if let Some(x) = args.update_event {
                    builder.add_update_event(x);
                }
                builder.finish()
            }

            #[inline]
            pub fn update_event(&self) -> Option<UpdateEvent<'a>> {
                // Safety:
                // Created from valid Table for this object
                // which contains a valid value in this slot
                unsafe {
                    self._tab.get::<flatbuffers::ForwardsUOffset<UpdateEvent>>(
                        UpdateEventMessage::VT_UPDATE_EVENT,
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
                        .get::<i32>(UpdateEventMessage::VT_MESSAGE_TYPE, Some(1))
                        .unwrap()
                }
            }
        }

        impl flatbuffers::Verifiable for UpdateEventMessage<'_> {
            #[inline]
            fn run_verifier(
                v: &mut flatbuffers::Verifier,
                pos: usize,
            ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
                use self::flatbuffers::Verifiable;
                v.visit_table(pos)?
                    .visit_field::<flatbuffers::ForwardsUOffset<UpdateEvent>>(
                        "update_event",
                        Self::VT_UPDATE_EVENT,
                        false,
                    )?
                    .visit_field::<i32>("message_type", Self::VT_MESSAGE_TYPE, false)?
                    .finish();
                Ok(())
            }
        }
        pub struct UpdateEventMessageArgs<'a> {
            pub update_event: Option<flatbuffers::WIPOffset<UpdateEvent<'a>>>,
            pub message_type: i32,
        }
        impl<'a> Default for UpdateEventMessageArgs<'a> {
            #[inline]
            fn default() -> Self {
                UpdateEventMessageArgs {
                    update_event: None,
                    message_type: 1,
                }
            }
        }

        pub struct UpdateEventMessageBuilder<'a: 'b, 'b> {
            fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
        }
        impl<'a: 'b, 'b> UpdateEventMessageBuilder<'a, 'b> {
            #[inline]
            pub fn add_update_event(
                &mut self,
                update_event: flatbuffers::WIPOffset<UpdateEvent<'b>>,
            ) {
                self.fbb_
                    .push_slot_always::<flatbuffers::WIPOffset<UpdateEvent>>(
                        UpdateEventMessage::VT_UPDATE_EVENT,
                        update_event,
                    );
            }
            #[inline]
            pub fn add_message_type(&mut self, message_type: i32) {
                self.fbb_
                    .push_slot::<i32>(UpdateEventMessage::VT_MESSAGE_TYPE, message_type, 1);
            }
            #[inline]
            pub fn new(
                _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
            ) -> UpdateEventMessageBuilder<'a, 'b> {
                let start = _fbb.start_table();
                UpdateEventMessageBuilder {
                    fbb_: _fbb,
                    start_: start,
                }
            }
            #[inline]
            pub fn finish(self) -> flatbuffers::WIPOffset<UpdateEventMessage<'a>> {
                let o = self.fbb_.end_table(self.start_);
                flatbuffers::WIPOffset::new(o.value())
            }
        }

        impl core::fmt::Debug for UpdateEventMessage<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut ds = f.debug_struct("UpdateEventMessage");
                ds.field("update_event", &self.update_event());
                ds.field("message_type", &self.message_type());
                ds.finish()
            }
        }
        #[inline]
        /// Verifies that a buffer of bytes contains a `UpdateEventMessage`
        /// and returns it.
        /// Note that verification is still experimental and may not
        /// catch every error, or be maximally performant. For the
        /// previous, unchecked, behavior use
        /// `root_as_update_event_message_unchecked`.
        pub fn root_as_update_event_message(
            buf: &[u8],
        ) -> Result<UpdateEventMessage, flatbuffers::InvalidFlatbuffer> {
            flatbuffers::root::<UpdateEventMessage>(buf)
        }
        #[inline]
        /// Verifies that a buffer of bytes contains a size prefixed
        /// `UpdateEventMessage` and returns it.
        /// Note that verification is still experimental and may not
        /// catch every error, or be maximally performant. For the
        /// previous, unchecked, behavior use
        /// `size_prefixed_root_as_update_event_message_unchecked`.
        pub fn size_prefixed_root_as_update_event_message(
            buf: &[u8],
        ) -> Result<UpdateEventMessage, flatbuffers::InvalidFlatbuffer> {
            flatbuffers::size_prefixed_root::<UpdateEventMessage>(buf)
        }
        #[inline]
        /// Verifies, with the given options, that a buffer of bytes
        /// contains a `UpdateEventMessage` and returns it.
        /// Note that verification is still experimental and may not
        /// catch every error, or be maximally performant. For the
        /// previous, unchecked, behavior use
        /// `root_as_update_event_message_unchecked`.
        pub fn root_as_update_event_message_with_opts<'b, 'o>(
            opts: &'o flatbuffers::VerifierOptions,
            buf: &'b [u8],
        ) -> Result<UpdateEventMessage<'b>, flatbuffers::InvalidFlatbuffer> {
            flatbuffers::root_with_opts::<UpdateEventMessage<'b>>(opts, buf)
        }
        #[inline]
        /// Verifies, with the given verifier options, that a buffer of
        /// bytes contains a size prefixed `UpdateEventMessage` and returns
        /// it. Note that verification is still experimental and may not
        /// catch every error, or be maximally performant. For the
        /// previous, unchecked, behavior use
        /// `root_as_update_event_message_unchecked`.
        pub fn size_prefixed_root_as_update_event_message_with_opts<'b, 'o>(
            opts: &'o flatbuffers::VerifierOptions,
            buf: &'b [u8],
        ) -> Result<UpdateEventMessage<'b>, flatbuffers::InvalidFlatbuffer> {
            flatbuffers::size_prefixed_root_with_opts::<UpdateEventMessage<'b>>(opts, buf)
        }
        #[inline]
        /// Assumes, without verification, that a buffer of bytes contains a UpdateEventMessage and returns it.
        /// # Safety
        /// Callers must trust the given bytes do indeed contain a valid `UpdateEventMessage`.
        pub unsafe fn root_as_update_event_message_unchecked(buf: &[u8]) -> UpdateEventMessage {
            flatbuffers::root_unchecked::<UpdateEventMessage>(buf)
        }
        #[inline]
        /// Assumes, without verification, that a buffer of bytes contains a size prefixed UpdateEventMessage and returns it.
        /// # Safety
        /// Callers must trust the given bytes do indeed contain a valid size prefixed `UpdateEventMessage`.
        pub unsafe fn size_prefixed_root_as_update_event_message_unchecked(
            buf: &[u8],
        ) -> UpdateEventMessage {
            flatbuffers::size_prefixed_root_unchecked::<UpdateEventMessage>(buf)
        }
        #[inline]
        pub fn finish_update_event_message_buffer<'a>(
            fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<UpdateEventMessage<'a>>,
        ) {
            fbb.finish(root, None);
        }

        #[inline]
        pub fn finish_size_prefixed_update_event_message_buffer<'a>(
            fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
            root: flatbuffers::WIPOffset<UpdateEventMessage<'a>>,
        ) {
            fbb.finish_size_prefixed(root, None);
        }
    } // pub mod UpdateEvents
} // pub mod Atrimo
