#[doc = "Register `WUPEN` reader"]
pub type R = crate::R<WupenSpec>;
#[doc = "Register `WUPEN` writer"]
pub type W = crate::W<WupenSpec>;
#[doc = "Field `EIRQWUEN` reader - desc EIRQWUEN"]
pub type EirqwuenR = crate::FieldReader<u16>;
#[doc = "Field `EIRQWUEN` writer - desc EIRQWUEN"]
pub type EirqwuenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SWDTWUEN` reader - desc SWDTWUEN"]
pub type SwdtwuenR = crate::BitReader;
#[doc = "Field `SWDTWUEN` writer - desc SWDTWUEN"]
pub type SwdtwuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD1WUEN` reader - desc PVD1WUEN"]
pub type Pvd1wuenR = crate::BitReader;
#[doc = "Field `PVD1WUEN` writer - desc PVD1WUEN"]
pub type Pvd1wuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD2WUEN` reader - desc PVD2WUEN"]
pub type Pvd2wuenR = crate::BitReader;
#[doc = "Field `PVD2WUEN` writer - desc PVD2WUEN"]
pub type Pvd2wuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPI0WUEN` reader - desc CMPI0WUEN"]
pub type Cmpi0wuenR = crate::BitReader;
#[doc = "Field `CMPI0WUEN` writer - desc CMPI0WUEN"]
pub type Cmpi0wuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKTMWUEN` reader - desc WKTMWUEN"]
pub type WktmwuenR = crate::BitReader;
#[doc = "Field `WKTMWUEN` writer - desc WKTMWUEN"]
pub type WktmwuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCALMWUEN` reader - desc RTCALMWUEN"]
pub type RtcalmwuenR = crate::BitReader;
#[doc = "Field `RTCALMWUEN` writer - desc RTCALMWUEN"]
pub type RtcalmwuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCPRDWUEN` reader - desc RTCPRDWUEN"]
pub type RtcprdwuenR = crate::BitReader;
#[doc = "Field `RTCPRDWUEN` writer - desc RTCPRDWUEN"]
pub type RtcprdwuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR0WUEN` reader - desc TMR0WUEN"]
pub type Tmr0wuenR = crate::BitReader;
#[doc = "Field `TMR0WUEN` writer - desc TMR0WUEN"]
pub type Tmr0wuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXWUEN` reader - desc RXWUEN"]
pub type RxwuenR = crate::BitReader;
#[doc = "Field `RXWUEN` writer - desc RXWUEN"]
pub type RxwuenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - desc EIRQWUEN"]
    #[inline(always)]
    pub fn eirqwuen(&self) -> EirqwuenR {
        EirqwuenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - desc SWDTWUEN"]
    #[inline(always)]
    pub fn swdtwuen(&self) -> SwdtwuenR {
        SwdtwuenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc PVD1WUEN"]
    #[inline(always)]
    pub fn pvd1wuen(&self) -> Pvd1wuenR {
        Pvd1wuenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc PVD2WUEN"]
    #[inline(always)]
    pub fn pvd2wuen(&self) -> Pvd2wuenR {
        Pvd2wuenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc CMPI0WUEN"]
    #[inline(always)]
    pub fn cmpi0wuen(&self) -> Cmpi0wuenR {
        Cmpi0wuenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc WKTMWUEN"]
    #[inline(always)]
    pub fn wktmwuen(&self) -> WktmwuenR {
        WktmwuenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc RTCALMWUEN"]
    #[inline(always)]
    pub fn rtcalmwuen(&self) -> RtcalmwuenR {
        RtcalmwuenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc RTCPRDWUEN"]
    #[inline(always)]
    pub fn rtcprdwuen(&self) -> RtcprdwuenR {
        RtcprdwuenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc TMR0WUEN"]
    #[inline(always)]
    pub fn tmr0wuen(&self) -> Tmr0wuenR {
        Tmr0wuenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - desc RXWUEN"]
    #[inline(always)]
    pub fn rxwuen(&self) -> RxwuenR {
        RxwuenR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc EIRQWUEN"]
    #[inline(always)]
    pub fn eirqwuen(&mut self) -> EirqwuenW<'_, WupenSpec> {
        EirqwuenW::new(self, 0)
    }
    #[doc = "Bit 16 - desc SWDTWUEN"]
    #[inline(always)]
    pub fn swdtwuen(&mut self) -> SwdtwuenW<'_, WupenSpec> {
        SwdtwuenW::new(self, 16)
    }
    #[doc = "Bit 17 - desc PVD1WUEN"]
    #[inline(always)]
    pub fn pvd1wuen(&mut self) -> Pvd1wuenW<'_, WupenSpec> {
        Pvd1wuenW::new(self, 17)
    }
    #[doc = "Bit 18 - desc PVD2WUEN"]
    #[inline(always)]
    pub fn pvd2wuen(&mut self) -> Pvd2wuenW<'_, WupenSpec> {
        Pvd2wuenW::new(self, 18)
    }
    #[doc = "Bit 19 - desc CMPI0WUEN"]
    #[inline(always)]
    pub fn cmpi0wuen(&mut self) -> Cmpi0wuenW<'_, WupenSpec> {
        Cmpi0wuenW::new(self, 19)
    }
    #[doc = "Bit 20 - desc WKTMWUEN"]
    #[inline(always)]
    pub fn wktmwuen(&mut self) -> WktmwuenW<'_, WupenSpec> {
        WktmwuenW::new(self, 20)
    }
    #[doc = "Bit 21 - desc RTCALMWUEN"]
    #[inline(always)]
    pub fn rtcalmwuen(&mut self) -> RtcalmwuenW<'_, WupenSpec> {
        RtcalmwuenW::new(self, 21)
    }
    #[doc = "Bit 22 - desc RTCPRDWUEN"]
    #[inline(always)]
    pub fn rtcprdwuen(&mut self) -> RtcprdwuenW<'_, WupenSpec> {
        RtcprdwuenW::new(self, 22)
    }
    #[doc = "Bit 23 - desc TMR0WUEN"]
    #[inline(always)]
    pub fn tmr0wuen(&mut self) -> Tmr0wuenW<'_, WupenSpec> {
        Tmr0wuenW::new(self, 23)
    }
    #[doc = "Bit 25 - desc RXWUEN"]
    #[inline(always)]
    pub fn rxwuen(&mut self) -> RxwuenW<'_, WupenSpec> {
        RxwuenW::new(self, 25)
    }
}
#[doc = "desc WUPEN\n\nYou can [`read`](crate::Reg::read) this register and get [`wupen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WupenSpec;
impl crate::RegisterSpec for WupenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wupen::R`](R) reader structure"]
impl crate::Readable for WupenSpec {}
#[doc = "`write(|w| ..)` method takes [`wupen::W`](W) writer structure"]
impl crate::Writable for WupenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WUPEN to value 0"]
impl crate::Resettable for WupenSpec {}
