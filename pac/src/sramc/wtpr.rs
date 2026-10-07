#[doc = "Register `WTPR` reader"]
pub type R = crate::R<WtprSpec>;
#[doc = "Register `WTPR` writer"]
pub type W = crate::W<WtprSpec>;
#[doc = "Field `WTPRC` reader - desc WTPRC"]
pub type WtprcR = crate::BitReader;
#[doc = "Field `WTPRC` writer - desc WTPRC"]
pub type WtprcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTPRKW` reader - desc WTPRKW"]
pub type WtprkwR = crate::FieldReader;
#[doc = "Field `WTPRKW` writer - desc WTPRKW"]
pub type WtprkwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - desc WTPRC"]
    #[inline(always)]
    pub fn wtprc(&self) -> WtprcR {
        WtprcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - desc WTPRKW"]
    #[inline(always)]
    pub fn wtprkw(&self) -> WtprkwR {
        WtprkwR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc WTPRC"]
    #[inline(always)]
    pub fn wtprc(&mut self) -> WtprcW<'_, WtprSpec> {
        WtprcW::new(self, 0)
    }
    #[doc = "Bits 1:7 - desc WTPRKW"]
    #[inline(always)]
    pub fn wtprkw(&mut self) -> WtprkwW<'_, WtprSpec> {
        WtprkwW::new(self, 1)
    }
}
#[doc = "desc WTPR\n\nYou can [`read`](crate::Reg::read) this register and get [`wtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WtprSpec;
impl crate::RegisterSpec for WtprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtpr::R`](R) reader structure"]
impl crate::Readable for WtprSpec {}
#[doc = "`write(|w| ..)` method takes [`wtpr::W`](W) writer structure"]
impl crate::Writable for WtprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WTPR to value 0"]
impl crate::Resettable for WtprSpec {}
