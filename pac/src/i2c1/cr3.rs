#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `TMOUTEN` reader - desc TMOUTEN"]
pub type TmoutenR = crate::BitReader;
#[doc = "Field `TMOUTEN` writer - desc TMOUTEN"]
pub type TmoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTMOUT` reader - desc LTMOUT"]
pub type LtmoutR = crate::BitReader;
#[doc = "Field `LTMOUT` writer - desc LTMOUT"]
pub type LtmoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTMOUT` reader - desc HTMOUT"]
pub type HtmoutR = crate::BitReader;
#[doc = "Field `HTMOUT` writer - desc HTMOUT"]
pub type HtmoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACKEN` reader - desc FACKEN"]
pub type FackenR = crate::BitReader;
#[doc = "Field `FACKEN` writer - desc FACKEN"]
pub type FackenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TMOUTEN"]
    #[inline(always)]
    pub fn tmouten(&self) -> TmoutenR {
        TmoutenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc LTMOUT"]
    #[inline(always)]
    pub fn ltmout(&self) -> LtmoutR {
        LtmoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HTMOUT"]
    #[inline(always)]
    pub fn htmout(&self) -> HtmoutR {
        HtmoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - desc FACKEN"]
    #[inline(always)]
    pub fn facken(&self) -> FackenR {
        FackenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TMOUTEN"]
    #[inline(always)]
    pub fn tmouten(&mut self) -> TmoutenW<'_, Cr3Spec> {
        TmoutenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc LTMOUT"]
    #[inline(always)]
    pub fn ltmout(&mut self) -> LtmoutW<'_, Cr3Spec> {
        LtmoutW::new(self, 1)
    }
    #[doc = "Bit 2 - desc HTMOUT"]
    #[inline(always)]
    pub fn htmout(&mut self) -> HtmoutW<'_, Cr3Spec> {
        HtmoutW::new(self, 2)
    }
    #[doc = "Bit 7 - desc FACKEN"]
    #[inline(always)]
    pub fn facken(&mut self) -> FackenW<'_, Cr3Spec> {
        FackenW::new(self, 7)
    }
}
#[doc = "desc CR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0x06"]
impl crate::Resettable for Cr3Spec {
    const RESET_VALUE: u32 = 0x06;
}
