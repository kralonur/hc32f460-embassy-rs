#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GrstctlSpec>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GrstctlSpec>;
#[doc = "Field `CSRST` reader - desc CSRST"]
pub type CsrstR = crate::BitReader;
#[doc = "Field `CSRST` writer - desc CSRST"]
pub type CsrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSRST` reader - desc HSRST"]
pub type HsrstR = crate::BitReader;
#[doc = "Field `HSRST` writer - desc HSRST"]
pub type HsrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCRST` reader - desc FCRST"]
pub type FcrstR = crate::BitReader;
#[doc = "Field `FCRST` writer - desc FCRST"]
pub type FcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFLSH` reader - desc RXFFLSH"]
pub type RxfflshR = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - desc RXFFLSH"]
pub type RxfflshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFLSH` reader - desc TXFFLSH"]
pub type TxfflshR = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - desc TXFFLSH"]
pub type TxfflshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - desc TXFNUM"]
pub type TxfnumR = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - desc TXFNUM"]
pub type TxfnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMAREQ` reader - desc DMAREQ"]
pub type DmareqR = crate::BitReader;
#[doc = "Field `AHBIDL` reader - desc AHBIDL"]
pub type AhbidlR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc CSRST"]
    #[inline(always)]
    pub fn csrst(&self) -> CsrstR {
        CsrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSRST"]
    #[inline(always)]
    pub fn hsrst(&self) -> HsrstR {
        HsrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc FCRST"]
    #[inline(always)]
    pub fn fcrst(&self) -> FcrstR {
        FcrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RXFFLSH"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RxfflshR {
        RxfflshR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TXFFLSH"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TxfflshR {
        TxfflshR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - desc TXFNUM"]
    #[inline(always)]
    pub fn txfnum(&self) -> TxfnumR {
        TxfnumR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - desc DMAREQ"]
    #[inline(always)]
    pub fn dmareq(&self) -> DmareqR {
        DmareqR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc AHBIDL"]
    #[inline(always)]
    pub fn ahbidl(&self) -> AhbidlR {
        AhbidlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CSRST"]
    #[inline(always)]
    pub fn csrst(&mut self) -> CsrstW<'_, GrstctlSpec> {
        CsrstW::new(self, 0)
    }
    #[doc = "Bit 1 - desc HSRST"]
    #[inline(always)]
    pub fn hsrst(&mut self) -> HsrstW<'_, GrstctlSpec> {
        HsrstW::new(self, 1)
    }
    #[doc = "Bit 2 - desc FCRST"]
    #[inline(always)]
    pub fn fcrst(&mut self) -> FcrstW<'_, GrstctlSpec> {
        FcrstW::new(self, 2)
    }
    #[doc = "Bit 4 - desc RXFFLSH"]
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RxfflshW<'_, GrstctlSpec> {
        RxfflshW::new(self, 4)
    }
    #[doc = "Bit 5 - desc TXFFLSH"]
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TxfflshW<'_, GrstctlSpec> {
        TxfflshW::new(self, 5)
    }
    #[doc = "Bits 6:10 - desc TXFNUM"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TxfnumW<'_, GrstctlSpec> {
        TxfnumW::new(self, 6)
    }
}
#[doc = "desc GRSTCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrstctlSpec;
impl crate::RegisterSpec for GrstctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstctl::R`](R) reader structure"]
impl crate::Readable for GrstctlSpec {}
#[doc = "`write(|w| ..)` method takes [`grstctl::W`](W) writer structure"]
impl crate::Writable for GrstctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x8000_0000"]
impl crate::Resettable for GrstctlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
