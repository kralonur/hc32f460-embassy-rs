#[doc = "Register `SNSEQCTLB3` reader"]
pub type R = crate::R<Snseqctlb3Spec>;
#[doc = "Register `SNSEQCTLB3` writer"]
pub type W = crate::W<Snseqctlb3Spec>;
#[doc = "Field `SNSDIST` reader - desc SNSDIST"]
pub type SnsdistR = crate::FieldReader<u32>;
#[doc = "Field `SNSDIST` writer - desc SNSDIST"]
pub type SnsdistW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `SNSCNTB` reader - desc SNSCNTB"]
pub type SnscntbR = crate::FieldReader<u16>;
#[doc = "Field `SNSCNTB` writer - desc SNSCNTB"]
pub type SnscntbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - desc SNSDIST"]
    #[inline(always)]
    pub fn snsdist(&self) -> SnsdistR {
        SnsdistR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - desc SNSCNTB"]
    #[inline(always)]
    pub fn snscntb(&self) -> SnscntbR {
        SnscntbR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - desc SNSDIST"]
    #[inline(always)]
    pub fn snsdist(&mut self) -> SnsdistW<'_, Snseqctlb3Spec> {
        SnsdistW::new(self, 0)
    }
    #[doc = "Bits 20:31 - desc SNSCNTB"]
    #[inline(always)]
    pub fn snscntb(&mut self) -> SnscntbW<'_, Snseqctlb3Spec> {
        SnscntbW::new(self, 20)
    }
}
#[doc = "desc SNSEQCTLB3\n\nYou can [`read`](crate::Reg::read) this register and get [`snseqctlb3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snseqctlb3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Snseqctlb3Spec;
impl crate::RegisterSpec for Snseqctlb3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snseqctlb3::R`](R) reader structure"]
impl crate::Readable for Snseqctlb3Spec {}
#[doc = "`write(|w| ..)` method takes [`snseqctlb3::W`](W) writer structure"]
impl crate::Writable for Snseqctlb3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SNSEQCTLB3 to value 0"]
impl crate::Resettable for Snseqctlb3Spec {}
