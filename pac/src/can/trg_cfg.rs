#[doc = "Register `TRG_CFG` reader"]
pub type R = crate::R<TrgCfgSpec>;
#[doc = "Register `TRG_CFG` writer"]
pub type W = crate::W<TrgCfgSpec>;
#[doc = "Field `TTPTR` reader - desc TTPTR"]
pub type TtptrR = crate::FieldReader;
#[doc = "Field `TTPTR` writer - desc TTPTR"]
pub type TtptrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TTYPE` reader - desc TTYPE"]
pub type TtypeR = crate::FieldReader;
#[doc = "Field `TTYPE` writer - desc TTYPE"]
pub type TtypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TEW` reader - desc TEW"]
pub type TewR = crate::FieldReader;
#[doc = "Field `TEW` writer - desc TEW"]
pub type TewW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - desc TTPTR"]
    #[inline(always)]
    pub fn ttptr(&self) -> TtptrR {
        TtptrR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - desc TTYPE"]
    #[inline(always)]
    pub fn ttype(&self) -> TtypeR {
        TtypeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - desc TEW"]
    #[inline(always)]
    pub fn tew(&self) -> TewR {
        TewR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc TTPTR"]
    #[inline(always)]
    pub fn ttptr(&mut self) -> TtptrW<'_, TrgCfgSpec> {
        TtptrW::new(self, 0)
    }
    #[doc = "Bits 8:10 - desc TTYPE"]
    #[inline(always)]
    pub fn ttype(&mut self) -> TtypeW<'_, TrgCfgSpec> {
        TtypeW::new(self, 8)
    }
    #[doc = "Bits 12:15 - desc TEW"]
    #[inline(always)]
    pub fn tew(&mut self) -> TewW<'_, TrgCfgSpec> {
        TewW::new(self, 12)
    }
}
#[doc = "desc TRG_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`trg_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trg_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrgCfgSpec;
impl crate::RegisterSpec for TrgCfgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`trg_cfg::R`](R) reader structure"]
impl crate::Readable for TrgCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`trg_cfg::W`](W) writer structure"]
impl crate::Writable for TrgCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRG_CFG to value 0"]
impl crate::Resettable for TrgCfgSpec {}
