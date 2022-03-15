#[doc = "Register `US_IMR_LON_MODE` reader"]
pub struct R(crate::R<US_IMR_LON_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_IMR_LON_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_IMR_LON_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_IMR_LON_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - RXRDY Interrupt Mask"]
pub struct RXRDY_R(crate::FieldReader<bool, bool>);
impl RXRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY` reader - TXRDY Interrupt Mask"]
pub struct TXRDY_R(crate::FieldReader<bool, bool>);
impl TXRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub struct OVRE_R(crate::FieldReader<bool, bool>);
impl OVRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEMPTY` reader - TXEMPTY Interrupt Mask"]
pub struct TXEMPTY_R(crate::FieldReader<bool, bool>);
impl TXEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIIC` reader - Ring Indicator Input Change Mask"]
pub struct RIIC_R(crate::FieldReader<bool, bool>);
impl RIIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRIC` reader - Data Set Ready Input Change Mask"]
pub struct DSRIC_R(crate::FieldReader<bool, bool>);
impl DSRIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSRIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDIC` reader - Data Carrier Detect Input Change Interrupt Mask"]
pub struct DCDIC_R(crate::FieldReader<bool, bool>);
impl DCDIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSFE` reader - LON Short Frame Error Interrupt Mask"]
pub struct LSFE_R(crate::FieldReader<bool, bool>);
impl LSFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCRCE` reader - LON CRC Error Interrupt Mask"]
pub struct LCRCE_R(crate::FieldReader<bool, bool>);
impl LCRCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCRCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCRCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTXD` reader - LON Transmission Done Interrupt Mask"]
pub struct LTXD_R(crate::FieldReader<bool, bool>);
impl LTXD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LTXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTXD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCOL` reader - LON Collision Interrupt Mask"]
pub struct LCOL_R(crate::FieldReader<bool, bool>);
impl LCOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFET` reader - LON Frame Early Termination Interrupt Mask"]
pub struct LFET_R(crate::FieldReader<bool, bool>);
impl LFET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LFET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LRXD` reader - LON Reception Done Interrupt Mask"]
pub struct LRXD_R(crate::FieldReader<bool, bool>);
impl LRXD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LRXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LRXD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBLOVFE` reader - LON Backlog Overflow Error Interrupt Mask"]
pub struct LBLOVFE_R(crate::FieldReader<bool, bool>);
impl LBLOVFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LBLOVFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBLOVFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Mask"]
    #[inline(always)]
    pub fn riic(&self) -> RIIC_R {
        RIIC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Mask"]
    #[inline(always)]
    pub fn dsric(&self) -> DSRIC_R {
        DSRIC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Mask"]
    #[inline(always)]
    pub fn dcdic(&self) -> DCDIC_R {
        DCDIC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LON Short Frame Error Interrupt Mask"]
    #[inline(always)]
    pub fn lsfe(&self) -> LSFE_R {
        LSFE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LON CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn lcrce(&self) -> LCRCE_R {
        LCRCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LON Transmission Done Interrupt Mask"]
    #[inline(always)]
    pub fn ltxd(&self) -> LTXD_R {
        LTXD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LON Collision Interrupt Mask"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LON Frame Early Termination Interrupt Mask"]
    #[inline(always)]
    pub fn lfet(&self) -> LFET_R {
        LFET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LON Reception Done Interrupt Mask"]
    #[inline(always)]
    pub fn lrxd(&self) -> LRXD_R {
        LRXD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LON Backlog Overflow Error Interrupt Mask"]
    #[inline(always)]
    pub fn lblovfe(&self) -> LBLOVFE_R {
        LBLOVFE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_imr_lon_mode](index.html) module"]
pub struct US_IMR_LON_MODE_SPEC;
impl crate::RegisterSpec for US_IMR_LON_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_imr_lon_mode::R](R) reader structure"]
impl crate::Readable for US_IMR_LON_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_IMR_LON_MODE to value 0"]
impl crate::Resettable for US_IMR_LON_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
