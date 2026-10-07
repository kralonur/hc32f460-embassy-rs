#[doc = "Register `DNSEQCTLB2` reader"]
pub type R = crate::R<Dnseqctlb2Spec>;
#[doc = "Register `DNSEQCTLB2` writer"]
pub type W = crate::W<Dnseqctlb2Spec>;
#[doc = "Field `DNSDIST` reader - desc DNSDIST"]
pub type DnsdistR = crate::FieldReader<u32>;
#[doc = "Field `DNSDIST` writer - desc DNSDIST"]
pub type DnsdistW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DNSCNTB` reader - desc DNSCNTB"]
pub type DnscntbR = crate::FieldReader<u16>;
#[doc = "Field `DNSCNTB` writer - desc DNSCNTB"]
pub type DnscntbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - desc DNSDIST"]
    #[inline(always)]
    pub fn dnsdist(&self) -> DnsdistR {
        DnsdistR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - desc DNSCNTB"]
    #[inline(always)]
    pub fn dnscntb(&self) -> DnscntbR {
        DnscntbR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - desc DNSDIST"]
    #[inline(always)]
    pub fn dnsdist(&mut self) -> DnsdistW<'_, Dnseqctlb2Spec> {
        DnsdistW::new(self, 0)
    }
    #[doc = "Bits 20:31 - desc DNSCNTB"]
    #[inline(always)]
    pub fn dnscntb(&mut self) -> DnscntbW<'_, Dnseqctlb2Spec> {
        DnscntbW::new(self, 20)
    }
}
#[doc = "desc DNSEQCTLB2\n\nYou can [`read`](crate::Reg::read) this register and get [`dnseqctlb2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dnseqctlb2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dnseqctlb2Spec;
impl crate::RegisterSpec for Dnseqctlb2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dnseqctlb2::R`](R) reader structure"]
impl crate::Readable for Dnseqctlb2Spec {}
#[doc = "`write(|w| ..)` method takes [`dnseqctlb2::W`](W) writer structure"]
impl crate::Writable for Dnseqctlb2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DNSEQCTLB2 to value 0"]
impl crate::Resettable for Dnseqctlb2Spec {}
